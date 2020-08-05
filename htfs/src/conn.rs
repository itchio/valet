use crate::rand_id::RandID;
use futures::{stream::TryStreamExt, AsyncRead, AsyncReadExt};
use reqwest::{header::HeaderMap, Response};
use std::{fmt, io, pin::Pin, task::Poll};

pub struct Conn {
    pub id: RandID,
    pub headers: HeaderMap,
    pub reader: Pin<Box<dyn AsyncRead + 'static>>,
    pub offset: u64,
}

impl Conn {
    pub fn new(res: Response, offset: u64) -> Self {
        Self {
            id: Default::default(),
            headers: res.headers().clone(),
            reader: Box::pin(into_reader(res)),
            offset,
        }
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.reader.read(buf).await;
        if let Ok(n) = &res {
            let n = *n;
            self.offset += n as u64;
        }
        res
    }
}

impl AsyncRead for Conn {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let res = self.reader.as_mut().poll_read(cx, buf);
        if let Poll::Ready(Ok(n)) = &res {
            self.offset += *n as u64;
        }
        res
    }
}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conn({:?} @ {})", self.id, self.offset)
    }
}

pub fn into_reader(res: reqwest::Response) -> impl AsyncRead {
    res.bytes_stream()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        .into_async_read()
}
