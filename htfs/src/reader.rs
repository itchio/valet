use futures::io::AsyncRead;
use futures::prelude::*;
use std::{
    fmt::{self, Debug},
    io,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub struct ReaderInner<R>
where
    R: AsyncRead + Unpin,
{
    reader: R,
    buf: Vec<u8>,
}

impl<R> ReaderInner<R>
where
    R: AsyncRead + Unpin,
{
    async fn private_read(mut self, n: usize) -> (Self, io::Result<usize>) {
        tracing::debug!("waiting...");
        tokio::time::delay_for(Duration::from_millis(200)).await;
        tracing::debug!("reading!");

        self.buf.clear();
        self.buf.reserve(n);
        for i in 0..n {
            self.buf.push(0);
        }

        let res = self.reader.read(&mut self.buf[..n]).await;
        (self, res)
    }
}

enum State<R>
where
    R: AsyncRead + Unpin + 'static,
{
    Idle(ReaderInner<R>),
    Pending(Pin<Box<dyn Future<Output = (ReaderInner<R>, io::Result<usize>)> + 'static>>),
    Transitional,
}

impl<R> fmt::Debug for State<R>
where
    R: AsyncRead + Unpin,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Idle(_) => write!(f, "Idle")?,
            State::Pending(_) => write!(f, "Pending")?,
            State::Transitional => write!(f, "Transitional")?,
        }
        Ok(())
    }
}

pub struct Reader2<R>
where
    R: AsyncRead + Unpin + 'static,
{
    state: State<R>,
}

impl<R> Reader2<R>
where
    R: AsyncRead + Unpin,
{
    pub fn new(reader: R) -> Self {
        Self {
            state: State::Idle(ReaderInner {
                reader,
                buf: Default::default(),
            }),
        }
    }
}

impl<R> Debug for Pin<&mut Reader2<R>>
where
    R: AsyncRead + Unpin,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reader(State={:?})", self.state)
    }
}

impl<R> AsyncRead for Reader2<R>
where
    R: AsyncRead + Unpin,
{
    #[tracing::instrument(skip(cx))]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = State::Transitional;
        std::mem::swap(&mut self.state, &mut state);
        let mut fut = match state {
            State::Idle(r) => Box::pin(r.private_read(buf.len())),
            State::Pending(fut) => fut,
            State::Transitional => unreachable!(),
        };
        let res = fut.as_mut().poll(cx);
        match res {
            Poll::Ready((inner, res)) => {
                let res = match res {
                    Ok(n) => {
                        for i in 0..n {
                            buf[i] = inner.buf[i]
                        }
                        Poll::Ready(Ok(n))
                    }
                    Err(e) => Poll::Ready(Err(e)),
                };
                self.state = State::Idle(inner);
                res
            }
            Poll::Pending => {
                self.state = State::Pending(fut);
                Poll::Pending
            }
        }
    }
}
