use async_std::sync::Arc;
use atomic_refcell::AtomicRefCell;

use crate::operator::Operator;

mod collect_vec;

pub trait Sink: Operator<()> {}

pub type StreamOutputRef<Out> = Arc<AtomicRefCell<Option<Out>>>;

pub struct StreamOutput<Out> {
    result: StreamOutputRef<Out>,
}

impl<Out> StreamOutput<Out> {
    pub fn get(self) -> Option<Out> {
        self.result.borrow_mut().take()
    }
}
