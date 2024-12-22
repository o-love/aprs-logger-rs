use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use tokio::io::{BufReader, Error, AsyncRead, AsyncBufReadExt};
use tokio::io;

struct StreamIterator<R, F, T>
where
    R: AsyncBufReadExt + Unpin,
    F: Fn(&[u8]) -> Option<T>,
{
    reader: R,
    processor: F,
}

impl<R, F, T> StreamIterator<R, F, T>
where
    R: AsyncBufReadExt + Unpin,
    F: Fn(&[u8]) -> Option<T>,
{
    fn new(reader: R, processor: F) -> Self {
        Self { reader, processor }
    }
}

impl<R, F, T> Stream for StreamIterator<R, F, T>
where
    R: AsyncBufReadExt + Unpin,
    F: Fn(&[u8]) -> Option<T> + Unpin,
{
    type Item = Result<T, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let self_mut = self.get_mut();

        let mut buf = Vec::new();
        match Pin::new(&mut self_mut.reader).poll_fill_buf(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(data)) => {
                if data.is_empty() {
                    // EOF
                    return Poll::Ready(None);
                }
                buf.extend_from_slice(data);
                Pin::new(&mut self_mut.reader).consume(data.len());

                // Process the line
                if let Some(value) = (self_mut.processor)(&buf) {
                    Poll::Ready(Some(Ok(value)))
                } else {
                    // Skip this line and poll next
                    self.poll_next(cx)
                }
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
        }
    }
}

pub fn process_stream<R, T, F>(input: R, processor: F) -> StreamIterator<R, F, T>
where
    R: AsyncRead + tokio::io::AsyncBufRead + std::marker::Unpin,
    F: Fn(&[u8]) -> Option<T> + 'static,
{
    StreamIterator::new(input, processor)
}
