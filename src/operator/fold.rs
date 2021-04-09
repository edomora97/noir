use std::sync::Arc;

use crate::block::NextStrategy;
use crate::operator::{Data, EndBlock, Operator, StreamElement, Timestamp};
use crate::scheduler::ExecutionMetadata;
use crate::stream::Stream;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Fold<NewOut: Data, PreviousOperators>
where
    PreviousOperators: Operator,
{
    prev: PreviousOperators,
    #[derivative(Debug = "ignore")]
    fold: Arc<dyn Fn(NewOut, PreviousOperators::Out) -> NewOut + Send + Sync>,
    init: NewOut,
    accumulator: Option<NewOut>,
    timestamp: Option<Timestamp>,
    max_watermark: Option<Timestamp>,
    received_end: bool,
}

impl<NewOut: Data, PreviousOperators: Operator> Fold<NewOut, PreviousOperators> {
    fn new<F>(prev: PreviousOperators, init: NewOut, fold: F) -> Self
    where
        F: Fn(NewOut, PreviousOperators::Out) -> NewOut + Send + Sync + 'static,
    {
        Fold {
            prev,
            fold: Arc::new(fold),
            init,
            accumulator: None,
            timestamp: None,
            max_watermark: None,
            received_end: false,
        }
    }
}

impl<NewOut: Data, PreviousOperators> Operator for Fold<NewOut, PreviousOperators>
where
    PreviousOperators: Operator + Send,
{
    type Out = NewOut;

    fn setup(&mut self, metadata: ExecutionMetadata) {
        self.prev.setup(metadata);
    }

    fn next(&mut self) -> StreamElement<NewOut> {
        while !self.received_end {
            match self.prev.next() {
                StreamElement::End => self.received_end = true,
                StreamElement::Watermark(ts) => {
                    self.max_watermark = Some(self.max_watermark.unwrap_or(ts).max(ts))
                }
                StreamElement::Item(item) => {
                    self.accumulator = Some((self.fold)(
                        self.accumulator.take().unwrap_or_else(|| self.init.clone()),
                        item,
                    ));
                }
                StreamElement::Timestamped(item, ts) => {
                    self.timestamp = Some(self.timestamp.unwrap_or(ts).max(ts));
                    self.accumulator = Some((self.fold)(
                        self.accumulator.take().unwrap_or_else(|| self.init.clone()),
                        item,
                    ));
                }
                // this block wont sent anything until the stream ends
                StreamElement::FlushBatch => {}
            }
        }

        // If there is an accumulated value, return it
        if let Some(acc) = self.accumulator.take() {
            if let Some(ts) = self.timestamp.take() {
                return StreamElement::Timestamped(acc, ts);
            } else {
                return StreamElement::Item(acc);
            }
        }

        // If watermark were received, send one downstream
        if let Some(ts) = self.max_watermark.take() {
            return StreamElement::Watermark(ts);
        }

        StreamElement::End
    }

    fn to_string(&self) -> String {
        format!(
            "{} -> Fold<{} -> {}>",
            self.prev.to_string(),
            std::any::type_name::<PreviousOperators::Out>(),
            std::any::type_name::<NewOut>()
        )
    }
}

impl<OperatorChain> Stream<OperatorChain>
where
    OperatorChain: Operator + Send + 'static,
{
    pub fn fold<NewOut: Data, F>(self, init: NewOut, f: F) -> Stream<impl Operator<Out = NewOut>>
    where
        F: Fn(NewOut, OperatorChain::Out) -> NewOut + Send + Sync + 'static,
    {
        let mut new_stream = self.add_block(EndBlock::new, NextStrategy::OnlyOne);
        // FIXME: when implementing Stream::max_parallelism use that here
        new_stream.block.scheduler_requirements.max_parallelism(1);
        new_stream.add_operator(|prev| Fold::new(prev, init, f))
    }

    pub fn fold_assoc<NewOut: Data, Local, Global>(
        self,
        init: NewOut,
        local: Local,
        global: Global,
    ) -> Stream<impl Operator<Out = NewOut>>
    where
        Local: Fn(NewOut, OperatorChain::Out) -> NewOut + Send + Sync + 'static,
        Global: Fn(NewOut, NewOut) -> NewOut + Send + Sync + 'static,
    {
        // Local fold
        let mut second_part = self
            .add_operator(|prev| Fold::new(prev, init.clone(), local))
            .add_block(EndBlock::new, NextStrategy::OnlyOne);

        // Global fold (which is done on only one node)
        // FIXME: when implementing Stream::max_parallelism use that here
        second_part.block.scheduler_requirements.max_parallelism(1);
        second_part.add_operator(|prev| Fold::new(prev, init, global))
    }
}

#[cfg(test)]
mod tests {

    use crate::config::EnvironmentConfig;
    use crate::environment::StreamEnvironment;
    use crate::operator::source;

    #[test]
    fn fold_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let res = env
            .stream(source)
            .fold("".to_string(), |s, n| s + &n.to_string())
            .collect_vec();
        env.execute();
        let res = res.get().unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "0123456789");
    }

    #[test]
    fn fold_assoc_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let res = env
            .stream(source)
            .fold_assoc("".to_string(), |s, n| s + &n.to_string(), |s1, s2| s1 + &s2)
            .collect_vec();
        env.execute();
        let res = res.get().unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "0123456789");
    }

    #[test]
    fn fold_shuffled_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let res = env
            .stream(source)
            .shuffle()
            .fold(Vec::new(), |mut v, n| {
                v.push(n);
                v
            })
            .collect_vec();
        env.execute();
        let mut res = res.get().unwrap();
        assert_eq!(res.len(), 1);
        res[0].sort_unstable();
        assert_eq!(res[0], (0..10u8).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn fold_assoc_shuffled_stream() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::IteratorSource::new(0..10u8);
        let res = env
            .stream(source)
            .shuffle()
            .fold_assoc(
                Vec::new(),
                |mut v, n| {
                    v.push(n);
                    v
                },
                |mut v1, mut v2| {
                    v2.append(&mut v1);
                    v2
                },
            )
            .collect_vec();
        env.execute();
        let mut res = res.get().unwrap();
        assert_eq!(res.len(), 1);
        res[0].sort_unstable();
        assert_eq!(res[0], (0..10u8).into_iter().collect::<Vec<_>>());
    }
}
