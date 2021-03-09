use core::iter::{IntoIterator, Iterator};
use std::collections::VecDeque;
use std::hash::Hash;
use std::iter::repeat;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

use crate::operator::source::SourceLoader;
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use crate::stream::{KeyValue, KeyedStream, Stream};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub(crate) struct Flatten<Out, IterOut, NewOut, PreviousOperators>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    IterOut: Iterator<Item = NewOut> + Clone + Send + 'static,
    PreviousOperators: Operator<Out>,
    NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    prev: PreviousOperators,
    // used to store elements that have not been returned by next() yet
    buffer: VecDeque<StreamElement<NewOut>>,
    // Make an element of type `Out` iterable
    // This is used to make `Flatten` behave differently when applied to `Stream` or `KeyedStream`
    // Takes `Out` as input, returns an `Iterator` with items of type `NewOut`
    #[derivative(Debug = "ignore")]
    make_iter: Arc<dyn Fn(Out) -> IterOut + Send + Sync>,
}

#[async_trait]
impl<Out, IterOut, NewOut, PreviousOperators> Operator<NewOut>
    for Flatten<Out, IterOut, NewOut, PreviousOperators>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    IterOut: Iterator<Item = NewOut> + Clone + Send + 'static,
    PreviousOperators: Operator<Out> + Send,
    NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn setup(&mut self, metadata: ExecutionMetadata) -> SourceLoader {
        self.prev.setup(metadata).await
    }

    fn next(&mut self) -> Option<StreamElement<NewOut>> {
        while self.buffer.is_empty() {
            match self.prev.next()? {
                StreamElement::Item(item) => {
                    self.buffer = (self.make_iter)(item).map(StreamElement::Item).collect()
                }
                StreamElement::Timestamped(item, ts) => {
                    self.buffer = (self.make_iter)(item)
                        .map(|value| StreamElement::Timestamped(value, ts))
                        .collect()
                }
                StreamElement::Watermark(ts) => return Some(StreamElement::Watermark(ts)),
                StreamElement::End => return Some(StreamElement::End),
            }
        }

        Some(self.buffer.pop_front().unwrap())
    }

    fn to_string(&self) -> String {
        format!(
            "{} -> Flatten<{} -> {}>",
            self.prev.to_string(),
            std::any::type_name::<Out>(),
            std::any::type_name::<NewOut>()
        )
    }
}

impl<In, Out, OperatorChain, NewOut> Stream<In, Out, OperatorChain>
where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: IntoIterator<Item = NewOut> + Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    <Out as IntoIterator>::IntoIter: Clone + Send + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
    NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn flatten(self) -> Stream<In, NewOut, impl Operator<NewOut>> {
        self.add_operator(|prev| Flatten {
            prev,
            buffer: Default::default(),
            // just convert `Out` to an `Iterator<Item = NewOut>`
            make_iter: Arc::new(|x| x.into_iter()),
        })
    }
}

impl<In, Out, OperatorChain> Stream<In, Out, OperatorChain>
where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    pub fn flat_map<MapOut, NewOut, F>(self, f: F) -> Stream<In, NewOut, impl Operator<NewOut>>
    where
        MapOut: IntoIterator<Item = NewOut>
            + Clone
            + Serialize
            + DeserializeOwned
            + Send
            + Sync
            + 'static,
        <MapOut as IntoIterator>::IntoIter: Clone + Send + 'static,
        NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
        F: Fn(Out) -> MapOut + Send + Sync + 'static,
    {
        self.map(f).flatten()
    }
}

impl<In, Key, Out, NewOut, OperatorChain> KeyedStream<In, Key, Out, OperatorChain>
where
    Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: IntoIterator<Item = NewOut> + Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    <Out as IntoIterator>::IntoIter: Clone + Send + 'static,
    NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<KeyValue<Key, Out>> + Send + 'static,
{
    pub fn flatten(self) -> KeyedStream<In, Key, NewOut, impl Operator<KeyValue<Key, NewOut>>> {
        self.add_operator(|prev| Flatten {
            prev,
            buffer: Default::default(),
            // convert a `KeyValue<Key, Out>` to an `Interator<Item = KeyValue<Key, NewOut>>`
            // repeat is used to have the same key for every value of type NewOut
            make_iter: Arc::new(move |(k, x)| repeat(k).zip(x.into_iter())),
        })
    }
}

impl<In, Key, Out, OperatorChain> KeyedStream<In, Key, Out, OperatorChain>
where
    Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<KeyValue<Key, Out>> + Send + 'static,
{
    pub fn flat_map<NewOut, MapOut, F>(
        self,
        f: F,
    ) -> KeyedStream<In, Key, NewOut, impl Operator<KeyValue<Key, NewOut>>>
    where
        MapOut: IntoIterator<Item = NewOut>
            + Clone
            + Serialize
            + DeserializeOwned
            + Send
            + Sync
            + 'static,
        <MapOut as IntoIterator>::IntoIter: Clone + Send + 'static,
        NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
        F: Fn(KeyValue<Key, Out>) -> MapOut + Send + Sync + 'static,
    {
        self.map(f).flatten()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use tokio::stream::from_iter;

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[tokio::test]
    async fn flatten_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(vec![
            vec![],
            vec![1u8, 2, 3],
            vec![4, 5],
            vec![],
            vec![6, 7, 8],
            vec![],
        ]));
        let res = env.stream(source).flatten().collect_vec();
        env.execute().await;
        assert_eq!(res.get().unwrap(), (1..=8).collect_vec());
    }

    #[tokio::test]
    async fn flatten_keyed_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..10u8));
        let res = env
            .stream(source)
            .group_by(|v| v % 2)
            .map(|(_k, v)| vec![v, v, v])
            .flatten()
            .unkey()
            .collect_vec();
        env.execute().await;
        let expected = (0..10u8)
            .flat_map(|x| vec![(x % 2, x), (x % 2, x), (x % 2, x)])
            .sorted()
            .collect_vec();
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        assert_eq!(expected, res);
    }

    #[tokio::test]
    async fn flat_map_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..10u8));
        let res = env
            .stream(source)
            .flat_map(|x| vec![x, 10 * x, 20 * x])
            .collect_vec();
        env.execute().await;
        let expected = (0..10u8)
            .flat_map(|x| vec![x, 10 * x, 20 * x])
            .collect_vec();
        assert_eq!(res.get().unwrap(), expected);
    }

    #[tokio::test]
    async fn flat_map_keyed_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..10u8));
        let res = env
            .stream(source)
            .group_by(|v| v % 2)
            .flat_map(|(_k, v)| vec![v, v, v])
            .unkey()
            .collect_vec();
        env.execute().await;
        let expected = (0..10u8)
            .flat_map(|x| vec![(x % 2, x), (x % 2, x), (x % 2, x)])
            .sorted()
            .collect_vec();
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        assert_eq!(expected, res);
    }
}
