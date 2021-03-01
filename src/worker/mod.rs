use async_std::channel::Receiver;

use crate::block::InnerBlock;
use crate::operator::Operator;
use crate::scheduler::{ExecutionMetadata, StartHandle};

pub fn spawn_worker<In, Out, OperatorChain>(
    block: InnerBlock<In, Out, OperatorChain>,
) -> StartHandle
where
    In: Send + 'static,
    Out: Send + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    let (sender, receiver) = async_std::channel::bounded(1);
    let join_handle = async_std::task::spawn(async move { worker(block, receiver).await });
    StartHandle {
        starter: sender,
        join_handle,
    }
}

async fn worker<In, Out, OperatorChain>(
    mut block: InnerBlock<In, Out, OperatorChain>,
    metadata_receiver: Receiver<ExecutionMetadata>,
) where
    In: Send + 'static,
    Out: Send + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    let metadata = metadata_receiver.recv().await.unwrap();
    block
        .execution_metadata
        .set(metadata)
        .map_err(|_| "Metadata already sent")
        .unwrap();
    drop(metadata_receiver);
    let metadata = block.execution_metadata.get().unwrap();
    info!(
        "Starting worker for {:?}: {}",
        metadata.coord,
        block.to_string(),
    );
    // TODO: call .next() and send to the next nodes
    block.operators.next().await;
}
