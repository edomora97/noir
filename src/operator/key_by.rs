use std::hash::Hash;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

use crate::operator::group_by::Keyer;
use crate::operator::source::SourceLoader;
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use crate::stream::{KeyValue, KeyedStream, Stream};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub(crate) struct KeyBy<Key, Out, OperatorChain>
where
    Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out>,
{
    prev: OperatorChain,
    #[derivative(Debug = "ignore")]
    keyer: Keyer<Key, Out>,
}

impl<Key, Out, OperatorChain> KeyBy<Key, Out, OperatorChain>
where
    Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out>,
{
    pub fn new(prev: OperatorChain, keyer: Keyer<Key, Out>) -> Self {
        Self { prev, keyer }
    }
}

#[async_trait]
impl<Key, Out, OperatorChain> Operator<KeyValue<Key, Out>> for KeyBy<Key, Out, OperatorChain>
where
    Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send,
{
    async fn setup(&mut self, metadata: ExecutionMetadata) -> SourceLoader {
        self.prev.setup(metadata).await
    }

    fn next(&mut self) -> Option<StreamElement<KeyValue<Key, Out>>> {
        Some(match self.prev.next()? {
            StreamElement::Item(t) => StreamElement::Item(((self.keyer)(&t), t)),
            StreamElement::Timestamped(t, ts) => {
                StreamElement::Timestamped(((self.keyer)(&t), t), ts)
            }
            StreamElement::Watermark(w) => StreamElement::Watermark(w),
            StreamElement::End => StreamElement::End,
        })
    }

    fn to_string(&self) -> String {
        format!(
            "{} -> KeyBy<{}>",
            self.prev.to_string(),
            std::any::type_name::<Key>(),
        )
    }
}

impl<In, Out, OperatorChain> Stream<In, Out, OperatorChain>
where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    pub fn key_by<Key, Keyer>(
        self,
        keyer: Keyer,
    ) -> KeyedStream<In, Key, Out, impl Operator<KeyValue<Key, Out>>>
    where
        Key: Clone + Serialize + DeserializeOwned + Send + Sync + Hash + Eq + 'static,
        Keyer: Fn(&Out) -> Key + Send + Sync + 'static,
    {
        let keyer = Arc::new(keyer);
        KeyedStream(self.add_operator(|prev| KeyBy::new(prev, keyer)))
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
    async fn key_by_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..10u8));
        let res = env.stream(source).key_by(|&n| n).unkey().collect_vec();
        env.execute().await;
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        let expected = (0..10u8).map(|n| (n, n)).collect_vec();
        assert_eq!(res, expected);
    }

    #[tokio::test]
    async fn key_by_stream2() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..100u8));
        let res = env
            .stream(source)
            .key_by(|&n| n.to_string().chars().next().unwrap())
            .unkey()
            .collect_vec();
        env.execute().await;
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        let expected = (0..100u8)
            .map(|n| (n.to_string().chars().next().unwrap(), n))
            .sorted()
            .collect_vec();
        assert_eq!(res, expected);
    }
}
