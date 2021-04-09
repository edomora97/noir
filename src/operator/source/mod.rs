pub use event_time_iterator::*;
pub use file::*;
pub use iterator::*;

use crate::operator::{Data, Operator};

mod event_time_iterator;
mod file;
mod iterator;

pub trait Source<Out: Data>: Operator {
    fn get_max_parallelism(&self) -> Option<usize>;
}
