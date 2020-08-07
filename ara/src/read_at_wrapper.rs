use crate::ReadAt;
use async_trait::async_trait;
use futures::{lock::Mutex, AsyncRead, AsyncReadExt};
use pin_project::pin_project;
use std::{
    collections::VecDeque,
    fmt, io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

/// Can build readers that read starting at a given offset
#[async_trait(?Send)]
pub trait GetReaderAt {
    type Reader: AsyncRead + Unpin;

    /// Returns a type reading the resource starting at `offset`
    async fn get_reader_at(self: &Arc<Self>, offset: u64) -> io::Result<Self::Reader>;
}

/// Wrapper that provides `ReadAt` from a type that implements `GetReaderAt`
pub struct ReadAtWrapper<Source>
where
    Source: GetReaderAt,
{
    heads: Mutex<VecDeque<Head<Source::Reader>>>,
    source: Arc<Source>,
    size: u64,
    max_heads: usize,
}

impl<Source> ReadAtWrapper<Source>
where
    Source: GetReaderAt,
{
    pub const DEFAULT_MAX_HEADS: usize = 2;

    pub fn new(
        source: Arc<Source>,
        size: u64,
        mut initial_head: Option<(u64, Source::Reader)>,
    ) -> Self {
        let mut heads: VecDeque<Head<Source::Reader>> = Default::default();
        if let Some((offset, reader)) = initial_head.take() {
            heads.push_back(Head { offset, reader });
        }

        Self {
            heads: Mutex::new(heads),
            source,
            size,
            max_heads: Self::DEFAULT_MAX_HEADS,
        }
    }

    async fn borrow_head(&self, offset: u64) -> io::Result<Head<Source::Reader>> {
        let mut heads = self.heads.lock().await;
        let candidate_index = heads
            .iter()
            .enumerate()
            .find(|(_, head)| head.offset == offset)
            .map(|(i, _)| i);

        let head = match candidate_index {
            Some(index) => {
                let head = heads
                    .remove(index)
                    .expect("internal logic error in heads pool manipulation");
                tracing::trace!("{:?}: borrowing", head);
                head
            }
            None => {
                drop(heads);
                let reader = self.source.get_reader_at(offset).await?;
                let head = Head { offset, reader };
                tracing::debug!("{:?}: new head", head);
                head
            }
        };

        Ok(head)
    }

    async fn return_head(&self, head: Head<Source::Reader>) {
        tracing::trace!("{:?}: returning", head);
        let mut heads = self.heads.lock().await;

        // returned heads are pushed to the back of the double-ended queue,
        // and expired heads are popped from the front, which effectively
        // functions as a cache with LRU (least-recently used) eviction policy
        heads.push_back(head);
        if heads.len() >= self.max_heads {
            heads.pop_front();
        }
    }
}

#[async_trait(?Send)]
impl<Source> ReadAt for ReadAtWrapper<Source>
where
    Source: GetReaderAt,
{
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        let mut head = self.borrow_head(offset).await?;
        // sic.: if this read fails, the head is considered unusable
        // and will not be returned. this can happen when dealing with
        // readers that are in fact network connections, and which can
        // expire, etc.
        let res = head.read(buf).await?;
        self.return_head(head).await;
        Ok(res)
    }

    fn size(&self) -> u64 {
        self.size
    }
}

#[pin_project]
struct Head<R>
where
    R: AsyncRead + Unpin,
{
    offset: u64,
    #[pin]
    reader: R,
}

impl<R> fmt::Debug for Head<R>
where
    R: AsyncRead + Unpin,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Head(offset = {})", self.offset)
    }
}

impl<R> AsyncRead for Head<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let head = self.project();

        let res = head.reader.poll_read(cx, buf);
        if let Poll::Ready(Ok(n)) = &res {
            *head.offset += *n as u64;
        }
        res
    }
}
