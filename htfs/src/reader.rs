use futures::io::AsyncRead;
use futures::{lock::Mutex, prelude::*};
use std::{
    fmt::{self, Debug},
    io,
    pin::Pin,
    sync::Arc,
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
        tracing::info!("waiting...");
        tokio::time::delay_for(Duration::from_millis(200)).await;
        tracing::info!("reading!");

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

#[derive(Debug)]
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

impl<R> AsyncRead for Reader2<R>
where
    R: AsyncRead + Unpin,
{
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

// ----

pub struct Reader<'a, R>
where
    R: AsyncRead + Unpin + 'a,
{
    pub reader: Arc<Mutex<R>>,
    pub fut: Option<Pin<Box<dyn Future<Output = io::Result<(Vec<u8>, usize)>> + 'a>>>,
}

impl<'a, R> Reader<'a, R>
where
    R: AsyncRead + Unpin + 'a,
{
    async fn private_read(reader: Arc<Mutex<R>>, mut buf: Vec<u8>) -> io::Result<(Vec<u8>, usize)> {
        tracing::info!("waiting...");
        tokio::time::delay_for(Duration::from_millis(200)).await;
        tracing::info!("reading!");

        let mut reader = reader.lock().await;
        match reader.read(&mut buf).await {
            Ok(n) => Ok((buf, n)),
            Err(e) => Err(e),
        }
    }
}

impl<'a, R> AsyncRead for Reader<'a, R>
where
    R: AsyncRead + Unpin + 'a,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        tracing::info!("reader::poll_read");

        let mut fut = match self.fut.take() {
            Some(fut) => {
                tracing::info!("polling existing");
                fut
            }
            None => {
                tracing::info!("making new future");
                let buf2 = vec![0u8; buf.len()];
                Box::pin(Self::private_read(self.reader.clone(), buf2))
            }
        };
        let res = fut.as_mut().poll(cx);
        self.fut = Some(fut);
        match res {
            Poll::Ready(res) => match res {
                Ok((buf2, n)) => {
                    for i in 0..n {
                        buf[i] = buf2[i]
                    }
                    Poll::Ready(Ok(n))
                }
                Err(e) => Poll::Ready(Err(e)),
            },
            Poll::Pending => Poll::Pending,
        }
    }
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, b);
            }
        }

        self.poll_read(cx, &mut [])
    }
}
