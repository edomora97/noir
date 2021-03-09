use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_stream::StreamExt;

use crate::operator::source::Source;
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct StreamSource<Out> {
    #[derivative(Debug = "ignore")]
    inner: Box<dyn tokio_stream::Stream<Item = Out> + Unpin + Send>,
}

impl<Out> StreamSource<Out> {
    pub fn new<S>(inner: S) -> Self
    where
        S: tokio_stream::Stream<Item = Out> + Unpin + Send + 'static,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<Out> Source<Out> for StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Unpin + 'static,
{
    fn get_max_parallelism(&self) -> Option<usize> {
        Some(1)
    }
}

#[async_trait]
impl<Out> Operator<Out> for StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Unpin + 'static,
{
    async fn setup(&mut self, _metadata: ExecutionMetadata) {}

    async fn next(&mut self) -> StreamElement<Out> {
        match self.inner.next().await {
            Some(t) => StreamElement::Item(t),
            None => StreamElement::End,
        }
    }

    fn to_string(&self) -> String {
        format!("StreamSource<{}>", std::any::type_name::<Out>())
    }
}

impl<Out> Clone for StreamSource<Out>
where
    Out: Send + Unpin + 'static,
{
    fn clone(&self) -> Self {
        // Since this is a non-parallel source, we don't want the other replicas to emit any value
        panic!("StreamSource cannot be cloned, max_parallelism should be 1");
    }
}
