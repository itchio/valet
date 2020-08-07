use crate::ReadAt;
use futures::io::AsyncRead;
use std::{
    future::Future,
    io,
    ops::Range,
    pin::Pin,
    task::{Context, Poll},
};

/// Exposes a section of a `ReadAt` as an `AsyncRead`
pub struct RangeReader<R>
where
    R: ReadAt,
{
    range: Range<u64>,
    state: State<R>,
}

enum State<R> {
    /// Waiting for read
    Idle((R, Vec<u8>)),
    /// Performing read
    Pending(Pin<Box<dyn Future<Output = (R, Vec<u8>, io::Result<usize>)> + 'static>>),
    /// Internal state for `poll_read` implementation
    Transitional,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("out of range: passed {range:?} but resource range is {resource_range:?}")]
    OutOfRange {
        range: Range<u64>,
        resource_range: Range<u64>,
    },
    #[error("backwards range passed: {0:?}")]
    BackwardsRange(Range<u64>),
}

impl<R> RangeReader<R>
where
    R: ReadAt + Unpin + 'static,
{
    pub const DEFAULT_BUF_SIZE: usize = 1024;

    pub fn new(inner: R, range: Range<u64>) -> Result<Self, Error> {
        Self::with_buf_size(inner, range, Self::DEFAULT_BUF_SIZE)
    }

    pub fn with_buf_size(inner: R, range: Range<u64>, bufsize: usize) -> Result<Self, Error> {
        if range.start > range.end {
            return Err(Error::BackwardsRange(range));
        }

        let resource_range = 0..inner.size();
        if !range.is_subset_of(&resource_range) {
            return Err(Error::OutOfRange {
                range,
                resource_range,
            });
        }

        let buf = vec![0u8; bufsize];
        Ok(Self {
            state: State::Idle((inner, buf)),
            range,
        })
    }
}

trait IsSubset {
    fn is_subset_of(&self, other: &Self) -> bool;
}

impl IsSubset for Range<u64> {
    fn is_subset_of(&self, other: &Self) -> bool {
        self.start >= other.end && self.end <= other.end
    }
}

impl<R> AsyncRead for RangeReader<R>
where
    R: ReadAt + Unpin + 'static,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = State::Transitional;
        std::mem::swap(&mut self.state, &mut state);
        let mut fut = match state {
            State::Idle((r, mut internal_buf)) => {
                let offset = self.range.start;
                let range_len = self.range.end - self.range.start;
                let read_size = std::cmp::min(range_len as usize, internal_buf.len());

                Box::pin(async move {
                    let res = r.read_at(offset, &mut internal_buf[..read_size]).await;
                    (r, internal_buf, res)
                })
            }
            State::Pending(fut) => fut,
            State::Transitional => unreachable!(),
        };
        let res = fut.as_mut().poll(cx);

        match res {
            Poll::Ready((inner, internal_buf, res)) => {
                if let Ok(bytes_read) = &res {
                    let bytes_read = *bytes_read;
                    for i in 0..bytes_read {
                        buf[i] = internal_buf[i]
                    }
                    self.range.start += bytes_read as u64;
                }
                self.state = State::Idle((inner, internal_buf));
                Poll::Ready(res)
            }
            Poll::Pending => {
                self.state = State::Pending(fut);
                Poll::Pending
            }
        }
    }
}
