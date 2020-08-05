use async_trait::async_trait;
use futures::AsyncRead;
use std::io;

/// Read bytes from any offset asynchronously
#[async_trait]
pub trait AsyncReadAt {
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize>;
}

#[async_trait(?Send)]
pub trait GetReaderAt {
    type Reader: AsyncRead;
    async fn get_reader_at(&self, offset: u64) -> io::Result<Self::Reader>;
}

pub trait IntoAsyncReadAt {
    type Out: AsyncReadAt;
    fn into_async_read_at() -> Self::Out;
}

impl<T> IntoAsyncReadAt for T
where
    T: GetReaderAt,
{
    type Out = Wrapper;
    fn into_async_read_at() -> Self::Out {
        Wrapper {}
    }
}

pub struct Wrapper {}

#[async_trait]
impl AsyncReadAt for Wrapper {
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}
