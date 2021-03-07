use std::time::Duration;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use batch_mode::*;

use crate::operator::source::SourceLoader;
use crate::scheduler::ExecutionMetadata;

mod batch_mode;
mod flatten;
mod fold;
mod group_by;
mod key_by;
mod keyed_fold;
mod map;
mod shuffle;
pub mod sink;
pub mod source;
pub(crate) mod start;
mod unkey;

/// When using timestamps and watermarks, this type expresses the timestamp of a message or of a
/// watermark.
pub type Timestamp = Duration;

/// An element of the stream. This is what enters and exits from the operators.
///
/// An operator may need to change the content of a `StreamElement` (e.g. a `Map` may change the
/// value of the `Item`). Usually `Watermark` and `End` are simply forwarded to the next operator in
/// the chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamElement<Out>
where
    Out: Clone + Send + Sync + 'static,
{
    /// A normal element containing just the value of the message.
    Item(Out),
    /// Like `Item`, but it's attached with a timestamp, it's used to ensure the ordering of the
    /// messages.
    Timestamped(Out, Timestamp),
    /// When an operator receives a `Watermark` with timestamp `t`, the operator will never see any
    /// message with timestamp less or equal to `t`.
    Watermark(Timestamp),
    /// The last message an operator will receive, indicating that the stream has ended.
    End,
}

/// An operator represents a unit of computation. It's always included inside a chain of operators,
/// inside a block.
///
/// Each operator implements the `Operator<Out>` trait, it produced a stream of `Out` elements.
///
/// An `Operator` must be Clone since it is part of a single chain when it's built, but it has to
/// be cloned to spawn the replicas of the block.
#[async_trait]
pub trait Operator<Out>: Clone
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Setup the source with the metadata and return the loader that tells to the source when to
    /// load a new batch.
    async fn setup(&mut self, metadata: ExecutionMetadata) -> SourceLoader;

    /// Take a value from the previous operator, process it and return it. Returns `None` when the
    /// current batch ends.
    fn next(&mut self) -> Option<StreamElement<Out>>;

    /// A string representation of the operator and its predecessors.
    fn to_string(&self) -> String;
}

impl<Out> StreamElement<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Change the type of the element inside the `StreamElement`.
    pub(crate) fn map<NewOut>(self, f: impl FnOnce(Out) -> NewOut) -> StreamElement<NewOut>
    where
        NewOut: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        match self {
            StreamElement::Item(item) => StreamElement::Item(f(item)),
            StreamElement::Timestamped(item, ts) => StreamElement::Timestamped(f(item), ts),
            StreamElement::Watermark(w) => StreamElement::Watermark(w),
            StreamElement::End => StreamElement::End,
        }
    }
}
