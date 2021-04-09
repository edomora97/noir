use std::sync::Arc;

use crate::operator::sink::Sink;
use crate::operator::{Data, DataKey, Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use crate::stream::{KeyValue, KeyedStream, Stream};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ForEachSink<Out: Data, PreviousOperators>
where
    PreviousOperators: Operator<Out = Out>,
{
    prev: PreviousOperators,
    #[derivative(Debug = "ignore")]
    f: Arc<dyn Fn(Out) + Send + Sync>,
}

impl<Out: Data, PreviousOperators> Operator for ForEachSink<Out, PreviousOperators>
where
    PreviousOperators: Operator<Out = Out> + Send,
{
    type Out = ();

    fn setup(&mut self, metadata: ExecutionMetadata) {
        self.prev.setup(metadata);
    }

    fn next(&mut self) -> StreamElement<()> {
        match self.prev.next() {
            StreamElement::Item(t) | StreamElement::Timestamped(t, _) => {
                (self.f)(t);
                StreamElement::Item(())
            }
            StreamElement::Watermark(w) => StreamElement::Watermark(w),
            StreamElement::End => StreamElement::End,
            StreamElement::FlushBatch => StreamElement::FlushBatch,
        }
    }

    fn to_string(&self) -> String {
        format!("{} -> ForEach", self.prev.to_string())
    }
}

impl<Out: Data, PreviousOperators> Sink for ForEachSink<Out, PreviousOperators> where
    PreviousOperators: Operator<Out = Out> + Send
{
}

impl<Out: Data, OperatorChain> Stream<Out, OperatorChain>
where
    OperatorChain: Operator<Out = Out> + Send + 'static,
{
    pub fn for_each<F: Fn(Out) + Send + Sync + 'static>(self, f: F) {
        self.add_operator(|prev| ForEachSink {
            prev,
            f: Arc::new(f),
        })
        .finalize_block();
    }
}

impl<Key: DataKey, Out: Data, OperatorChain> KeyedStream<Key, Out, OperatorChain>
where
    OperatorChain: Operator<Out = KeyValue<Key, Out>> + Send + 'static,
{
    pub fn for_each<F: Fn(Key, Out) + Send + Sync + 'static>(self, f: F) {
        self.0.for_each(move |(key, out)| f(key, out))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::sync::Arc;

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[test]
    fn for_each() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let sum = Arc::new(AtomicU8::new(0));
        let sum2 = sum.clone();
        env.stream(source).for_each(move |x| {
            sum.fetch_add(x, Ordering::Release);
        });
        env.execute();
        assert_eq!(sum2.load(Ordering::Acquire), (0..10).sum::<u8>());
    }

    #[test]
    fn for_each_keyed() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let sum = Arc::new(AtomicU8::new(0));
        let sum2 = sum.clone();
        env.stream(source)
            .group_by(|x| x % 2)
            .for_each(move |p, x| {
                sum.fetch_add(x * (p + 1), Ordering::Release);
            });
        env.execute();
        assert_eq!(
            sum2.load(Ordering::Acquire),
            (0..10).map(|x| x * (x % 2 + 1)).sum::<u8>()
        );
    }
}
