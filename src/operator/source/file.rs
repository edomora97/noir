use async_std::channel::{Receiver, Sender};
use async_std::fs::File;
use async_std::io::prelude::SeekExt;
use async_std::io::{BufReader, SeekFrom};
use async_std::path::PathBuf;
use async_std::prelude::*;
use async_std::task::spawn;
use async_trait::async_trait;

use crate::operator::source::{Source, SourceBatch, SourceLoader};
use crate::operator::{Operator, StreamElement};
use crate::scheduler::ExecutionMetadata;

#[derive(Debug)]
pub struct FileSource {
    path: PathBuf,
    batch: SourceBatch<String>,
}

impl FileSource {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into(),
            batch: Default::default(),
        }
    }
}

async fn source_body(
    next_batch: Receiver<()>,
    next_batch_done: Sender<()>,
    mut current: usize,
    end: usize,
    mut reader: BufReader<File>,
    batch: SourceBatch<String>,
) {
    while let Ok(()) = next_batch.recv().await {
        let element = if current <= end {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(len) if len > 0 => {
                    current += len;
                    StreamElement::Item(line)
                }
                Ok(_) => StreamElement::End,
                Err(e) => panic!(e),
            }
        } else {
            StreamElement::End
        };

        batch.borrow_mut().push_back(element);

        next_batch_done.send(()).await.unwrap();
    }
}

impl Source<String> for FileSource {
    fn get_max_parallelism(&self) -> Option<usize> {
        None
    }
}

#[async_trait]
impl Operator<String> for FileSource {
    async fn setup(&mut self, metadata: ExecutionMetadata) -> SourceLoader {
        let global_id = metadata.global_id;
        let num_replicas = metadata.num_replicas;

        let file = File::open(&self.path)
            .await
            .expect("FileSource: error while opening file");
        let file_size = file.metadata().await.unwrap().len() as usize;

        let range_size = file_size / num_replicas;
        let start = range_size * global_id;
        let mut current = start;
        let end = if global_id == num_replicas - 1 {
            file_size
        } else {
            start + range_size
        };

        let mut reader = BufReader::new(file);
        // Seek reader to the first byte to be read
        reader
            .seek(SeekFrom::Current(start as i64))
            .await
            .expect("seek file");
        if global_id != 0 {
            // discard first line
            let mut s = String::new();
            current += reader
                .read_line(&mut s)
                .await
                .expect("Cannot read line from file");
        }

        let (source_loader, start_loading, done_loading) = SourceLoader::new();
        let batch = self.batch.clone();
        spawn(async move {
            source_body(start_loading, done_loading, current, end, reader, batch).await
        });
        source_loader
    }

    fn next(&mut self) -> Option<StreamElement<String>> {
        self.batch.borrow_mut().pop_front()
    }

    fn to_string(&self) -> String {
        format!("FileSource<{}>", self.path.display())
    }
}

impl Clone for FileSource {
    fn clone(&self) -> Self {
        FileSource {
            path: self.path.clone(),
            // the batch must be different, otherwise all the clones will point to the same one
            batch: Default::default(),
        }
    }
}
