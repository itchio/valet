use async_trait::async_trait;
use std::{io, sync::Arc};

pub mod buf_reader_at;
pub mod range_reader;
pub mod read_at_wrapper;

/// Provides size and asynchronous random access to a resource
#[async_trait(?Send)]
pub trait ReadAt {
    /// Read bytes from resource, starting at `offset`, into `buf`
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize>;

    /// Reads exactly `buf`, starting at `offset`
    async fn read_at_exact(&self, mut offset: u64, mut buf: &mut [u8]) -> io::Result<()> {
        while !buf.is_empty() {
            let n = self.read_at(offset, buf).await?;
            offset += n as u64;
            buf = &mut buf[n..];
        }
        Ok(())
    }

    /// Returns the size of the resource
    fn size(&self) -> u64;
}

/// Type that can be converted into an `AsyncReadAt`.
pub trait AsAsyncReadAt {
    type Out: ReadAt;

    fn as_async_read_at(self: &Arc<Self>) -> Self::Out;
}
