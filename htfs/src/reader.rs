use crate::{conn::Conn, response_reader, File};
use futures::io::AsyncRead;
use futures::prelude::*;
use reqwest::Method;
use std::{
    fmt::{self, Debug},
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

pub struct ReaderInner {
    file: Arc<File>,
    offset: u64,
    buf: Vec<u8>,
}

fn make_io_error<E>(e: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

impl ReaderInner {
    async fn read_internal(&mut self, n: usize) -> io::Result<usize> {
        self.buf.clear();
        self.buf.reserve(n);
        for i in 0..n {
            self.buf.push(0);
        }

        let mut conn = {
            let mut conns = self.file.connections.lock().await;
            let candidate = conns.iter().enumerate().find_map(|(i, conn)| {
                if conn.offset == self.offset {
                    Some(i)
                } else {
                    None
                }
            });

            match candidate {
                Some(index) => {
                    let conn = conns.remove(index);
                    tracing::debug!("re-using conn {:?}", conn.id);
                    conn
                }
                None => {
                    tracing::debug!("making fresh conn");
                    let req = self
                        .file
                        .client
                        .request(Method::GET, self.file.url.clone())
                        .header("range", format!("bytes={}-", self.offset))
                        .build()
                        .map_err(make_io_error)?;
                    let res = self.file.client.execute(req).map_err(make_io_error).await?;
                    let reader = response_reader::as_reader(res);
                    let conn = Conn::new(reader, self.offset);
                    tracing::debug!("made fresh conn {:?}", conn.id);
                    conn
                }
            }
        };

        let res = conn.read(&mut self.buf[..n]).await;
        {
            let mut conns = self.file.connections.lock().await;
            conns.push(conn);
        }

        if let Ok(n) = &res {
            self.offset += *n as u64;
        }
        res
    }
}

enum State {
    Idle(ReaderInner),
    Pending(Pin<Box<dyn Future<Output = (ReaderInner, io::Result<usize>)> + 'static>>),
    Transitional,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Idle(_) => write!(f, "Idle")?,
            State::Pending(_) => write!(f, "Pending")?,
            State::Transitional => write!(f, "Transitional")?,
        }
        Ok(())
    }
}

pub struct Reader2 {
    state: State,
}

impl Reader2 {
    pub fn new(file: Arc<File>, offset: u64) -> Self {
        Self {
            state: State::Idle(ReaderInner {
                file,
                offset,
                buf: Default::default(),
            }),
        }
    }
}

impl Debug for Pin<&mut Reader2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reader(State={:?})", self.state)
    }
}

impl AsyncRead for Reader2 {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = State::Transitional;
        std::mem::swap(&mut self.state, &mut state);
        let mut fut = match state {
            State::Idle(mut r) => {
                let len = buf.len();
                Box::pin(async move {
                    let res = r.read_internal(len).await;
                    (r, res)
                })
            }
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
