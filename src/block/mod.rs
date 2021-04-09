use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub use batcher::BatchMode;
pub(crate) use batcher::*;
pub(crate) use next_strategy::*;

use crate::operator::{Data, Operator};
use crate::stream::BlockId;

mod batcher;
mod next_strategy;

/// A chain of operators that will be run inside the same host. The block takes as input elements of
/// type `In` and produces elements of type `Out`.
///
/// The type `In` is used to make sure the blocks are connected following the correct type.
///
/// `OperatorChain` is the type of the chain of operators inside the block. It must be an operator
/// that yields values of type `Out`.
#[derive(Debug, Clone)]
pub(crate) struct InnerBlock<Out: Data, OperatorChain>
where
    OperatorChain: Operator<Out = Out>,
{
    /// The identifier of the block inside the environment.
    pub(crate) id: BlockId,
    /// The current chain of operators.
    pub(crate) operators: OperatorChain,
    /// The batch mode of this block.
    pub(crate) batch_mode: BatchMode,
    /// The set of requirements that the block imposes on the scheduler.
    pub(crate) scheduler_requirements: SchedulerRequirements,

    pub _out_type: PhantomData<Out>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SchedulerRequirements {
    /// If some of the operators inside the chain require a limit on the parallelism of this node,
    /// it is stored here. `None` means that the scheduler is allowed to spawn as many copies of
    /// this block as it likes.
    ///
    /// The value specified is only an upper bound, the scheduler is allowed to spawn less blocks,
    pub(crate) max_parallelism: Option<usize>,
}

impl<Out: Data, OperatorChain> InnerBlock<Out, OperatorChain>
where
    OperatorChain: Operator<Out = Out>,
{
    pub fn new(id: BlockId, operators: OperatorChain, batch_mode: BatchMode) -> Self {
        Self {
            id,
            operators,
            batch_mode,
            scheduler_requirements: Default::default(),
            _out_type: Default::default(),
        }
    }
}

impl<Out: Data, OperatorChain> Display for InnerBlock<Out, OperatorChain>
where
    OperatorChain: Operator<Out = Out>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operators.to_string())
    }
}

impl SchedulerRequirements {
    /// Limit the maximum parallelism of this block.
    pub(crate) fn max_parallelism(&mut self, max_parallelism: usize) {
        if let Some(old) = self.max_parallelism {
            self.max_parallelism = Some(old.min(max_parallelism));
        } else {
            self.max_parallelism = Some(max_parallelism);
        }
    }
}
