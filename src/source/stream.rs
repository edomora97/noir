use async_std::stream::Stream;
use async_std::stream::StreamExt;
use async_trait::async_trait;

use crate::operator::{Operator, StreamElement};
use crate::source::Source;

pub struct StreamSource<Out> {
    inner: Box<dyn Stream<Item = Out> + Unpin + Send>,
}

impl<Out> StreamSource<Out> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Stream<Item = Out> + 'static + Unpin + Send,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<Out> Source<Out> for StreamSource<Out> {}

#[async_trait]
impl<Out> Operator<Out> for StreamSource<Out> {
    async fn next(&mut self) -> StreamElement<Out> {
        let next = self.inner.next();
        let next = next.await;
        match next {
            Some(t) => StreamElement::Item(t),
            None => StreamElement::End,
        }
    }
}