use std::collections::HashMap;

use async_std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::network::{Coord, NetworkMessage, NetworkSender};
use crate::scheduler::ExecutionMetadata;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};

/// The list with the interesting senders of a single block.
#[derive(Debug, Clone)]
pub(crate) struct SenderList(pub Vec<Coord>);

/// The next strategy used at the end of a block.
///
/// A block in the job graph may have many next blocks. Each of them will receive the message, which
/// of their replica will receive it depends on the value of the next strategy.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub(crate) enum NextStrategy<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Only one of the replicas will receive the message:
    ///
    /// - if the block is not replicated, the only replica will receive the message
    /// - if the next block is replicated as much as the current block the corresponding replica
    ///   will receive the message
    /// - otherwise the execution graph is malformed  
    OnlyOne,
    /// A random replica will receive the message.
    Random,
    /// Among the next replica, the one is selected based on the hash of the key of the message.
    GroupBy(#[derivative(Debug = "ignore")] Arc<dyn Fn(&Out) -> usize + Send + Sync>),
}

impl<Out> NextStrategy<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Convert a `NextStrategy` of a type to another. `GroupBy` cannot be converted since it should
    /// be used only at the end of the block.
    ///
    /// The `From` trait cannot be implemented since it clashes with the auto implementation
    /// `From<NextStrategy<A>> for NextStrategy<A>` inside the `core` crate.
    pub fn into<NewOut>(self) -> NextStrategy<NewOut>
    where
        NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        match self {
            NextStrategy::OnlyOne => NextStrategy::OnlyOne,
            NextStrategy::Random => NextStrategy::Random,
            NextStrategy::GroupBy(_) => {
                panic!("Cannot convert a NextStrategy::GroupBy from a type to another")
            }
        }
    }

    /// Group the senders from a block using the current next strategy.
    ///
    /// The returned value is a list of `SenderList`s, one for each next block in the execution
    /// graph. The messages will be sent to one replica of each group, according to the strategy.
    pub fn group_senders(
        &self,
        metadata: &ExecutionMetadata,
        senders: &HashMap<Coord, NetworkSender<NetworkMessage<Out>>>,
    ) -> Vec<SenderList> {
        let mut by_block_id: HashMap<_, Vec<_>> = HashMap::new();
        for (coord, sender) in senders {
            by_block_id.entry(coord.block_id).or_default().push(sender);
        }
        let mut senders = Vec::new();
        for (block_id, block_senders) in by_block_id {
            let block_senders = block_senders.iter().map(|s| s.coord).sorted().collect_vec();
            match self {
                NextStrategy::OnlyOne => {
                    assert!(
                        block_senders.len() == 1 || block_senders.len() == metadata.num_replicas,
                        "OnlyOne cannot mix the number of replicas: block {} -> {}, replicas {} -> {}",
                        metadata.coord.block_id,
                        block_id,
                        block_senders.len(),
                        metadata.num_replicas
                    );
                    if block_senders.len() == 1 {
                        senders.push(SenderList(block_senders));
                    } else {
                        let mut found = false;
                        for sender in block_senders {
                            if sender.replica_id == metadata.coord.replica_id {
                                found = true;
                                senders.push(SenderList(vec![sender]));
                                break;
                            }
                        }
                        assert!(
                            found,
                            "Cannot found next sender for the block with the same replica_id: {}",
                            metadata.coord
                        );
                    }
                }
                NextStrategy::Random | NextStrategy::GroupBy(_) => {
                    senders.push(SenderList(block_senders))
                }
            }
        }
        senders
    }
}

impl<Out> NextStrategy<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Compute the index of the replica which this message should be forwarded to.
    pub fn index(&self, message: &Out) -> usize {
        match self {
            NextStrategy::OnlyOne => 0,
            NextStrategy::Random => thread_rng().gen(),
            NextStrategy::GroupBy(keyer) => keyer(message),
        }
    }
}

impl<Out> Display for NextStrategy<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NextStrategy::OnlyOne => write!(f, "OnlyOne"),
            NextStrategy::Random => write!(f, "Random"),
            NextStrategy::GroupBy(_) => write!(f, "GroupBy"),
        }
    }
}
