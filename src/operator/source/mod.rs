use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use file::*;
pub use stream::*;

use crate::operator::{Operator, StreamElement};
use async_std::channel::{bounded, Receiver, Sender};
use async_std::sync::Arc;
use atomic_refcell::AtomicRefCell;
use std::collections::VecDeque;

mod file;
mod stream;

pub(crate) type SourceBatch<Out> = Arc<AtomicRefCell<VecDeque<StreamElement<Out>>>>;

#[async_trait]
pub trait Source<Out>: Operator<Out>
where
    Out: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn get_max_parallelism(&self) -> Option<usize>;
}

pub struct SourceLoader {
    start_loading: Sender<()>,
    done_loading: Receiver<()>,
}

impl SourceLoader {
    pub fn new() -> (SourceLoader, Receiver<()>, Sender<()>) {
        let (start_loading, start_loading_recv) = bounded(1);
        let (done_loading, done_loading_recv) = bounded(1);
        (
            SourceLoader {
                start_loading,
                done_loading: done_loading_recv,
            },
            start_loading_recv,
            done_loading,
        )
    }

    pub(crate) async fn load(&self) {
        self.start_loading.send(()).await.unwrap();
        self.done_loading
            .recv()
            .await
            .expect("Failed to load batch");
    }
}
