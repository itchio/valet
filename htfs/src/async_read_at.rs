use async_trait::async_trait;
use futures::{AsyncRead, AsyncReadExt, Future};
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

/// Read bytes from any offset asynchronously
#[async_trait(?Send)]
pub trait AsyncReadAt {
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize>;
    fn size(&self) -> u64;
}

#[async_trait(?Send)]
pub trait GetReaderAt {
    type Reader: AsyncRead;
    async fn get_reader_at(&self, offset: u64) -> io::Result<Self::Reader>;
    fn size(&self) -> u64;
}

pub trait IntoAsyncReadAt {
    type Out: AsyncReadAt;

    fn into_async_read_at(self) -> Self::Out;
}

impl<T> IntoAsyncReadAt for T
where
    T: GetReaderAt,
    <T as GetReaderAt>::Reader: std::marker::Unpin,
{
    type Out = Wrapper<T>;
    fn into_async_read_at(self) -> Self::Out {
        Wrapper { inner: self }
    }
}

pub struct Wrapper<R> {
    inner: R,
}

#[async_trait(?Send)]
impl<R> AsyncReadAt for Wrapper<R>
where
    R: GetReaderAt,
    <R as GetReaderAt>::Reader: std::marker::Unpin,
{
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.get_reader_at(offset).await?.read(buf).await
    }
    fn size(&self) -> u64 {
        self.inner.size()
    }
}

enum State<R> {
    Idle((R, Vec<u8>)),
    Pending(Pin<Box<dyn Future<Output = (R, Vec<u8>, io::Result<usize>)> + 'static>>),
    Transitional,
}

pub struct AsyncSectionReader<R>
where
    R: AsyncReadAt,
{
    offset: u64,
    len: u64,
    state: State<R>,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum AsyncSectionReaderError {
    #[error("offset too large: passed {passed}, but maximum is {max}")]
    OffsetTooLarge { passed: u64, max: u64 },
    #[error("length too large: passed {passed}, but maximum is {max}")]
    LenTooLarge { passed: u64, max: u64 },
}

impl<R> AsyncSectionReader<R>
where
    R: AsyncReadAt + Unpin + 'static,
{
    pub fn new(inner: R, offset: u64, len: u64) -> Result<Self, AsyncSectionReaderError> {
        let size = inner.size();
        if offset > size {
            return Err(AsyncSectionReaderError::OffsetTooLarge {
                passed: offset,
                max: size,
            });
        }
        let maxlen = size - offset;
        if len > maxlen {
            return Err(AsyncSectionReaderError::LenTooLarge {
                passed: len,
                max: maxlen,
            });
        }

        let buf = vec![0u8; 1024];
        Ok(Self {
            state: State::Idle((inner, buf)),
            offset,
            len,
        })
    }

    pub fn into_inner(self) -> R {
        match self.state {
            State::Idle((r, _internal_buf)) => r,
            State::Pending(_) => todo!(),
            State::Transitional => unreachable!(),
        }
    }
}

impl<R> AsyncRead for AsyncSectionReader<R>
where
    R: AsyncReadAt + Unpin + 'static,
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
                let offset = self.offset;
                let read_size = std::cmp::min(self.len as usize, internal_buf.len());

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
                if let Ok(n) = &res {
                    let n = *n;
                    for i in 0..n {
                        buf[i] = internal_buf[i]
                    }
                    let n = n as u64;
                    self.offset += n;
                    self.len -= n;
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
