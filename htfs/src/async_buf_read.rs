use async_trait::async_trait;
use futures::{AsyncRead, AsyncReadExt};
use std::io;

/// Read bytes from any offset asynchronously
#[async_trait(?Send)]
pub trait AsyncReadAt {
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize>;
}

#[async_trait(?Send)]
pub trait GetReaderAt {
    type Reader: AsyncRead;
    async fn get_reader_at(&self, offset: u64) -> io::Result<Self::Reader>;
}

pub trait IntoAsyncReadAt: Send {
    type Out: AsyncReadAt;

    fn into_async_read_at(self) -> Self::Out;
}

impl<T> IntoAsyncReadAt for T
where
    T: GetReaderAt + Send,
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
}
