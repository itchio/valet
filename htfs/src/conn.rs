use futures::{AsyncRead, AsyncReadExt};
use std::{io, pin::Pin};

pub struct Conn<'a> {
    pub inner: Pin<Box<dyn AsyncRead + 'a>>,
    pub offset: u64,
}

impl<'a> Conn<'a> {
    pub fn new<R>(inner: R, offset: u64) -> Self
    where
        R: AsyncRead + 'a,
    {
        Self {
            inner: Box::pin(inner),
            offset,
        }
    }

    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.inner.read(buf).await;
        if let Ok(n) = &res {
            let n = *n;
            self.offset += n as u64;
        }
        res
    }
}
