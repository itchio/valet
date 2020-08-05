use crate::rand_id::RandID;
use futures::{AsyncRead, AsyncReadExt};
use std::{fmt, io, pin::Pin};

pub struct Conn<'a> {
    pub id: RandID,
    pub inner: Pin<Box<dyn AsyncRead + 'a>>,
    pub offset: u64,
}

impl<'a> Conn<'a> {
    pub fn new<R>(inner: R, offset: u64) -> Self
    where
        R: AsyncRead + 'a,
    {
        Self {
            id: Default::default(),
            inner: Box::pin(inner),
            offset,
        }
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.inner.read(buf).await;
        if let Ok(n) = &res {
            let n = *n;
            self.offset += n as u64;
        }
        res
    }
}

impl<'a> fmt::Debug for Conn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conn({:?} @ {})", self.id, self.offset)
    }
}
