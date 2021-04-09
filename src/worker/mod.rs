use crate::block::InnerBlock;
use crate::channel::BoundedChannelReceiver;
use crate::operator::{Operator, StreamElement};
use crate::scheduler::{ExecutionMetadata, StartHandle};

pub(crate) fn spawn_worker<OperatorChain>(block: InnerBlock<OperatorChain>) -> StartHandle
where
    OperatorChain: Operator + Send + 'static,
{
    let (sender, receiver) = BoundedChannelReceiver::new(1);
    let join_handle = std::thread::Builder::new()
        .name(format!("Block{}", block.id))
        .spawn(move || worker(block, receiver))
        .unwrap();
    StartHandle::new(sender, join_handle)
}

fn worker<OperatorChain>(
    mut block: InnerBlock<OperatorChain>,
    metadata_receiver: BoundedChannelReceiver<ExecutionMetadata>,
) where
    OperatorChain: Operator + Send + 'static,
{
    let metadata = metadata_receiver.recv().unwrap();
    drop(metadata_receiver);
    info!(
        "Starting worker for {}: {}",
        metadata.coord,
        block.to_string(),
    );
    // notify the operators that we are about to start
    block.operators.setup(metadata.clone());
    while !matches!(block.operators.next(), StreamElement::End) {
        // nothing to do
    }
    info!("Worker {} completed, exiting", metadata.coord);
}
