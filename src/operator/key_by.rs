use crate::operator::{DataKey, Keyer};
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use crate::stream::{KeyValue, KeyedStream, Stream};
use std::sync::Arc;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct KeyBy<Key: DataKey, OperatorChain>
where
    OperatorChain: Operator,
{
    prev: OperatorChain,
    #[derivative(Debug = "ignore")]
    keyer: Keyer<Key, OperatorChain::Out>,
}

impl<Key: DataKey, OperatorChain> KeyBy<Key, OperatorChain>
where
    OperatorChain: Operator,
{
    pub fn new(prev: OperatorChain, keyer: Keyer<Key, OperatorChain::Out>) -> Self {
        Self { prev, keyer }
    }
}

impl<Key: DataKey, OperatorChain> Operator for KeyBy<Key, OperatorChain>
where
    OperatorChain: Operator + Send,
{
    type Out = KeyValue<Key, OperatorChain::Out>;

    fn setup(&mut self, metadata: ExecutionMetadata) {
        self.prev.setup(metadata);
    }

    fn next(&mut self) -> StreamElement<KeyValue<Key, OperatorChain::Out>> {
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

impl<OperatorChain> Stream<OperatorChain>
where
    OperatorChain: Operator + Send + 'static,
{
    pub fn key_by<Key: DataKey, Keyer>(
        self,
        keyer: Keyer,
    ) -> KeyedStream<Key, OperatorChain::Out, impl Operator<Out = KeyValue<Key, OperatorChain::Out>>>
    where
        Keyer: Fn(&OperatorChain::Out) -> Key + Send + Sync + 'static,
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
