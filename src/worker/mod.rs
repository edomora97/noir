use async_std::channel::Receiver;
use async_std::task::spawn;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::block::{Batcher, InnerBlock};
use crate::operator::{Operator, StreamElement};
use crate::scheduler::{ExecutionMetadata, StartHandle};
use std::collections::HashMap;

pub(crate) fn spawn_worker<In, Out, OperatorChain>(
    block: InnerBlock<In, Out, OperatorChain>,
) -> StartHandle
where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    let (sender, receiver) = async_std::channel::bounded(1);
    let join_handle = spawn(async move { worker(block, receiver).await });
    StartHandle::new(sender, join_handle)
}

async fn worker<In, Out, OperatorChain>(
    mut block: InnerBlock<In, Out, OperatorChain>,
    metadata_receiver: Receiver<ExecutionMetadata>,
) where
    In: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    OperatorChain: Operator<Out> + Send + 'static,
{
    let metadata = metadata_receiver.recv().await.unwrap();
    drop(metadata_receiver);
    info!(
        "Starting worker for {}: {}",
        metadata.coord,
        block.to_string(),
    );
    // notify the operators that we are about to start
    let loader = block.operators.setup(metadata.clone()).await;

    let senders = metadata.network.lock().await.get_senders(metadata.coord);
    let sender_groups = block.next_strategy.group_senders(&metadata, &senders);
    let mut senders: HashMap<_, _> = senders
        .into_iter()
        .map(|(coord, sender)| (coord, Batcher::new(sender, block.batch_mode)))
        .collect();

    'main_loop: loop {
        // load a batch from the previous block
        loader.load().await;

        // process all the messages of the batch
        while let Some(message) = block.operators.next() {
            let mut to_send = Vec::new();
            match &message {
                StreamElement::Watermark(_) | StreamElement::End => {
                    for senders in sender_groups.iter() {
                        for &sender in senders.0.iter() {
                            to_send.push((message.clone(), sender))
                        }
                    }
                }
                StreamElement::Item(item) | StreamElement::Timestamped(item, _) => {
                    let index = block.next_strategy.index(&item);
                    for sender in sender_groups.iter() {
                        let index = index % sender.0.len();
                        to_send.push((message.clone(), sender.0[index]));
                    }
                }
            };
            for (message, coord) in to_send {
                senders[&coord].enqueue(message).await;
            }

            if matches!(message, StreamElement::End) {
                for (_, batcher) in senders.drain() {
                    batcher.end().await;
                }
                // stop the worker when End is received
                break 'main_loop;
            }
        }
    }
    info!("Worker {} completed, exiting", metadata.coord);
}
