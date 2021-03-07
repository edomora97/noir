use async_std::stream;
use async_std::stream::StreamExt;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::operator::source::{Source, SourceBatch, SourceLoader};
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;
use async_std::channel::{Receiver, Sender};
use async_std::task::spawn;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    batch: SourceBatch<Out>,
    #[derivative(Debug = "ignore")]
    source_loader: Option<SourceLoader>,
}

impl<Out> StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new<S>(inner: S) -> Self
    where
        S: stream::Stream<Item = Out> + Unpin + Send + 'static,
    {
        let batch: SourceBatch<Out> = Default::default();
        let (source_loader, start_loading, done_loading) = SourceLoader::new();
        let batch2 = batch.clone();
        spawn(async move { source_body(start_loading, done_loading, inner, batch2).await });
        Self {
            batch,
            source_loader: Some(source_loader),
        }
    }
}

async fn source_body<Out, S>(
    next_batch: Receiver<()>,
    next_batch_done: Sender<()>,
    mut stream: S,
    batch: SourceBatch<Out>,
) where
    S: stream::Stream<Item = Out> + Unpin + Send + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    while let Ok(()) = next_batch.recv().await {
        let element = match stream.next().await {
            Some(t) => StreamElement::Item(t),
            None => StreamElement::End,
        };
        batch.borrow_mut().push_back(element);
        next_batch_done.send(()).await.unwrap();
    }
}

impl<Out> Source<Out> for StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn get_max_parallelism(&self) -> Option<usize> {
        Some(1)
    }
}

#[async_trait]
impl<Out> Operator<Out> for StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn setup(&mut self, _metadata: ExecutionMetadata) -> SourceLoader {
        self.source_loader.take().unwrap()
    }

    fn next(&mut self) -> Option<StreamElement<Out>> {
        self.batch.borrow_mut().pop_front()
    }

    fn to_string(&self) -> String {
        format!("StreamSource<{}>", std::any::type_name::<Out>())
    }
}

impl<Out> Clone for StreamSource<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        // Since this is a non-parallel source, we don't want the other replicas to emit any value
        panic!("StreamSource cannot be cloned, max_parallelism should be 1");
    }
}
