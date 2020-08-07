use async_trait::async_trait;
use std::{io, sync::Arc};

pub mod range_reader;
pub mod read_at_wrapper;

/// Provides size and asynchronous random access to a resource
#[async_trait(?Send)]
pub trait ReadAt {
    /// Read bytes from resource, starting at `offset`, into `buf`
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<usize>;

    /// Returns the size of the resource
    fn size(&self) -> u64;
}

/// Type that can be converted into an `AsyncReadAt`.
pub trait AsAsyncReadAt {
    type Out: ReadAt;

    fn as_async_read_at(self: &Arc<Self>) -> Self::Out;
}
