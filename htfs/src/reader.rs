use futures::io::AsyncRead;
use futures::prelude::*;
use std::{
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};

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
    async fn private_read(
        reader: Arc<Mutex<R>>,
        mut buf: Vec<u8>,
    ) -> std::io::Result<(Vec<u8>, usize)> {
        tracing::info!("waiting...");
        tokio::time::delay_for(Duration::from_millis(200)).await;
        tracing::info!("reading!");

        let mut reader = reader.lock().unwrap();
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
    ) -> Poll<std::io::Result<usize>> {
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
