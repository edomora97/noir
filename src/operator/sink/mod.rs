use std::sync::Arc;

use crate::operator::Operator;
use tokio::sync::Mutex;

mod collect_vec;

pub trait Sink: Operator<()> {}

pub type StreamOutputRef<Out> = Arc<Mutex<Option<Out>>>;

pub struct StreamOutput<Out> {
    result: StreamOutputRef<Out>,
}

impl<Out> StreamOutput<Out> {
    pub fn get(self) -> Option<Out> {
        self.result
            .try_lock()
            .expect("Cannot lock output result")
            .take()
    }
}
