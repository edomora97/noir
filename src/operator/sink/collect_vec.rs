use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::operator::sink::{Sink, StreamOutput, StreamOutputRef};
use crate::operator::source::SourceLoader;
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use crate::stream::Stream;

#[derive(Debug)]
pub(crate) struct CollectVecSink<Out, PreviousOperators>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    PreviousOperators: Operator<Out>,
{
    prev: PreviousOperators,
    result: Option<Vec<Out>>,
    output: StreamOutputRef<Vec<Out>>,
}

#[async_trait]
impl<Out, PreviousOperators> Operator<()> for CollectVecSink<Out, PreviousOperators>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    PreviousOperators: Operator<Out> + Send,
{
    async fn setup(&mut self, metadata: ExecutionMetadata) -> SourceLoader {
        self.prev.setup(metadata).await
    }

    fn next(&mut self) -> Option<StreamElement<()>> {
        Some(match self.prev.next()? {
            StreamElement::Item(t) | StreamElement::Timestamped(t, _) => {
                // cloned CollectVecSink or already ended stream
                if let Some(result) = self.result.as_mut() {
                    result.push(t);
                }
                StreamElement::Item(())
            }
            StreamElement::Watermark(w) => StreamElement::Watermark(w),
            StreamElement::End => {
                if let Some(result) = self.result.take() {
                    *self.output.borrow_mut() = Some(result);
                }
                StreamElement::End
            }
        })
    }

    fn to_string(&self) -> String {
        format!("{} -> CollectVecSink", self.prev.to_string())
    }
}

impl<Out, PreviousOperators> Sink for CollectVecSink<Out, PreviousOperators>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    PreviousOperators: Operator<Out> + Send,
{
}

impl<Out, PreviousOperators> Clone for CollectVecSink<Out, PreviousOperators>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    PreviousOperators: Operator<Out> + Send,
{
    fn clone(&self) -> Self {
        panic!("CollectVecSink cannot be cloned, max_parallelism should be 1");
    }
}

impl<In, Out, OperatorChain> Stream<In, Out, OperatorChain>
where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    pub fn collect_vec(self) -> StreamOutput<Vec<Out>> {
        let output = StreamOutputRef::default();
        let mut new_stream = self.add_block();
        // FIXME: when implementing Stream::max_parallelism use that here
        new_stream.block.scheduler_requirements.max_parallelism(1);
        new_stream
            .add_operator(|prev| CollectVecSink {
                prev,
                result: Some(Vec::new()),
                output: output.clone(),
            })
            .finalize_block();
        StreamOutput { result: output }
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
    async fn collect_vec() {
        let mut env = StreamEnvironment::new(EnvironmentConfig::local(4));
        let source = source::StreamSource::new(from_iter(0..10u8));
        let res = env.stream(source).collect_vec();
        env.execute().await;
        assert_eq!(res.get().unwrap(), (0..10).collect_vec());
    }
}
