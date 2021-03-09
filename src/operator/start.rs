use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::spawn;

use crate::network::{NetworkMessage, NetworkReceiver};
use crate::operator::source::{Source, SourceBatch, SourceLoader};
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;

#[derive(Debug, Derivative)]
#[derivative(Default(bound = ""))]
pub(crate) struct StartBlock<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    batch: SourceBatch<Out>,
}

async fn source_body<Out>(
    mut next_batch: Receiver<()>,
    next_batch_done: Sender<()>,
    metadata: ExecutionMetadata,
    mut receiver: NetworkReceiver<NetworkMessage<Out>>,
    batch: SourceBatch<Out>,
) where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    let mut missing_ends = metadata.num_prev;

    while let Some(()) = next_batch.recv().await {
        let mut buf = receiver.recv().await.unwrap();
        let last_message = buf
            .get(buf.len() - 1)
            .expect("Previous block sent an empty message");
        if matches!(last_message, StreamElement::End) {
            missing_ends -= 1;
            debug!(
                "{} received an end, {} more to come",
                metadata.coord, missing_ends
            );

            if missing_ends == 0 {
                // all the previous blocks sent and end: we're done
                info!("StartBlock for {} has ended", metadata.coord);
            } else {
                // avoid sending the End if we don't have received an End from everyone
                buf.pop().unwrap();
            }
        }

        let mut batch = batch.borrow_mut();
        batch.append(&mut buf.into());
        // make sure the borrow ends before sending the "done" signal
        drop(batch);

        next_batch_done.send(()).await.unwrap();
    }
}

impl<Out> Source<Out> for StartBlock<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn get_max_parallelism(&self) -> Option<usize> {
        None
    }
}

#[async_trait]
impl<Out> Operator<Out> for StartBlock<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn setup(&mut self, metadata: ExecutionMetadata) -> SourceLoader {
        let batch = self.batch.clone();
        let mut network = metadata.network.lock().await;
        let receiver = network.get_receiver(metadata.coord);
        drop(network);
        debug!(
            "StartBlock {} initialized, {} previous blocks, receiver is: {:?}",
            metadata.coord, metadata.num_prev, receiver
        );

        let (source_loader, start_loading, done_loading) = SourceLoader::new();
        spawn(
            async move { source_body(start_loading, done_loading, metadata, receiver, batch).await },
        );
        source_loader
    }

    fn next(&mut self) -> Option<StreamElement<Out>> {
        self.batch.borrow_mut().pop_front()
    }

    fn to_string(&self) -> String {
        format!("[{}]", std::any::type_name::<Out>())
    }
}

impl<Out> Clone for StartBlock<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            batch: Default::default(),
        }
    }
}
