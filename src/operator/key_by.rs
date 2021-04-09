use crate::operator::{Data, DataKey, Keyer};
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use crate::stream::{KeyValue, KeyedStream, Stream};
use std::sync::Arc;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct KeyBy<Key: DataKey, Out: Data, OperatorChain>
where
    OperatorChain: Operator<Out = Out>,
{
    prev: OperatorChain,
    #[derivative(Debug = "ignore")]
    keyer: Keyer<Key, Out>,
}

impl<Key: DataKey, Out: Data, OperatorChain> KeyBy<Key, Out, OperatorChain>
where
    OperatorChain: Operator<Out = Out>,
{
    pub fn new(prev: OperatorChain, keyer: Keyer<Key, Out>) -> Self {
        Self { prev, keyer }
    }
}

impl<Key: DataKey, Out: Data, OperatorChain> Operator for KeyBy<Key, Out, OperatorChain>
where
    OperatorChain: Operator<Out = Out> + Send,
{
    type Out = KeyValue<Key, Out>;

    fn setup(&mut self, metadata: ExecutionMetadata) {
        self.prev.setup(metadata);
    }

    fn next(&mut self) -> StreamElement<KeyValue<Key, Out>> {
        match self.prev.next() {
            StreamElement::Item(t) => StreamElement::Item(((self.keyer)(&t), t)),
            StreamElement::Timestamped(t, ts) => {
                StreamElement::Timestamped(((self.keyer)(&t), t), ts)
            }
            StreamElement::Watermark(w) => StreamElement::Watermark(w),
            StreamElement::End => StreamElement::End,
            StreamElement::FlushBatch => StreamElement::FlushBatch,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{} -> KeyBy<{}>",
            self.prev.to_string(),
            std::any::type_name::<Key>(),
        )
    }
}

impl<Out: Data, OperatorChain> Stream<Out, OperatorChain>
where
    OperatorChain: Operator<Out = Out> + Send + 'static,
{
    pub fn key_by<Key: DataKey, Keyer>(
        self,
        keyer: Keyer,
    ) -> KeyedStream<Key, Out, impl Operator<Out = KeyValue<Key, Out>>>
    where
        Keyer: Fn(&Out) -> Key + Send + Sync + 'static,
    {
        let keyer = Arc::new(keyer);
        KeyedStream(self.add_operator(|prev| KeyBy::new(prev, keyer)))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[test]
    fn key_by_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let res = env.stream(source).key_by(|&n| n).unkey().collect_vec();
        env.execute();
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        let expected = (0..10u8).map(|n| (n, n)).collect_vec();
        assert_eq!(res, expected);
    }

    #[test]
    fn key_by_stream2() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..100u8);
        let res = env
            .stream(source)
            .key_by(|&n| n.to_string().chars().next().unwrap())
            .unkey()
            .collect_vec();
        env.execute();
        let res = res.get().unwrap().into_iter().sorted().collect_vec();
        let expected = (0..100u8)
            .map(|n| (n.to_string().chars().next().unwrap(), n))
            .sorted()
            .collect_vec();
        assert_eq!(res, expected);
    }
}
