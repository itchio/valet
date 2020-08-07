use crate::ReadAt;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures::lock::Mutex;
use lru_time_cache::LruCache;
use std::io;

pub struct BufReaderAt<R>
where
    R: ReadAt,
{
    inner: R,
    layout: PageLayout,
    cache: Mutex<LruCache<u64, Bytes>>,
}

pub struct BufReaderAtOpts {
    /// Cached block size
    page_size: u64,
    /// Capacity of page cache
    max_cached_pages: usize,
}

impl Default for BufReaderAtOpts {
    fn default() -> Self {
        Self {
            page_size: 256 * 1024, // 256 KiB
            max_cached_pages: 32,
        }
    }
}

impl<R> BufReaderAt<R>
where
    R: ReadAt,
{
    pub fn new(inner: R) -> Self {
        Self::with_opts(inner, Default::default())
    }

    pub fn with_opts(inner: R, opts: BufReaderAtOpts) -> Self {
        Self {
            cache: Mutex::new(LruCache::with_capacity(opts.max_cached_pages)),
            layout: PageLayout {
                resource_size: inner.size(),
                page_size: opts.page_size,
            },
            inner,
        }
    }
}

#[async_trait(?Send)]
impl<R> ReadAt for BufReaderAt<R>
where
    R: ReadAt,
{
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<usize> {
        let page_info = self.layout.page_at(offset).map_err(make_io_error)?;
        let read_size = std::cmp::min(buf.len(), page_info.remaining() as usize);

        let mut cache = self.cache.lock().await;
        if let Some(page_bytes) = cache.get(&page_info.number) {
            for i in 0..read_size {
                buf[i] = page_bytes[page_info.offset_in_page as usize + i];
            }
            return Ok(read_size);
        } else {
            let mut page_bytes = BytesMut::with_capacity(page_info.size as _);
            unsafe {
                page_bytes.set_len(page_info.size as _);
            }
            self.inner
                .read_at(page_info.page_start(), page_bytes.as_mut())
                .await?;

            for i in 0..read_size {
                buf[i] = page_bytes[page_info.offset_in_page as usize + i];
            }

            cache.insert(page_info.number, page_bytes.into());
            return Ok(read_size);
        }
    }

    fn size(&self) -> u64 {
        self.layout.resource_size
    }
}

fn make_io_error<E: std::error::Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageLayout {
    resource_size: u64,
    page_size: u64,
}

#[derive(Debug, thiserror::Error)]
enum PageError {
    #[error("out of bounds: requested offset {requested} > resource size {resource_size}")]
    OutOfBounds { requested: u64, resource_size: u64 },
}

impl PageLayout {
    /// Returns information for the page at a given offset, or an error
    /// if out of bounds.
    fn page_at(self, offset: u64) -> Result<PageInfo, PageError> {
        if offset > self.resource_size {
            return Err(PageError::OutOfBounds {
                requested: offset,
                resource_size: self.resource_size,
            });
        }

        let number = offset / self.page_size;
        let offset_in_page = offset - number * self.page_size;

        let end = (number + 1) * self.page_size;
        let size = if end > self.resource_size {
            let page_start = number * self.page_size;
            self.resource_size - page_start
        } else {
            self.page_size
        };

        Ok(PageInfo {
            number,
            offset_in_page,
            size,
            layout: self,
        })
    }
}

/// Page-aware position information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageInfo {
    /// Number of the page. For 1024-byte pages, page 0
    /// is bytes 0..1024, page 1 is bytes 1024..2048, etc.
    number: u64,

    /// Offset within page itself. For 1024-byte pages,
    /// page 1 with offset 10 is byte 1034.
    offset_in_page: u64,

    /// Actual size of this page, may be less than `max_page_size`
    /// if this is the last page and the size of the resource
    /// is not a multiple of `max_page_size`.
    size: u64,

    /// How the resource is divided into pages
    layout: PageLayout,
}

impl PageInfo {
    /// Returns the number of bytes that remain in this page.
    /// For example, page 0 with offset 1014 has 10 bytes remaining
    /// (for 1024-byte pages).
    fn remaining(self) -> u64 {
        self.size - self.offset_in_page
    }

    /// Returns the offset at which this page starts in the resouce
    /// Page 2 starts at offset 2048 (for 1024-byte pages).
    fn page_start(self) -> u64 {
        self.number * self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_page_layout() {
        let layout = PageLayout {
            page_size: 100,
            resource_size: 328,
        };

        assert!(layout.page_at(0).is_ok());
        assert!(layout.page_at(128).is_ok());
        assert!(layout.page_at(328).is_ok());

        assert!(layout.page_at(329).is_err());
        assert!(layout.page_at(350).is_err());

        assert_eq!(
            layout.page_at(0).unwrap(),
            PageInfo {
                number: 0,
                offset_in_page: 0,
                size: 100,
                layout,
            }
        );

        assert_eq!(
            layout.page_at(99).unwrap(),
            PageInfo {
                number: 0,
                offset_in_page: 99,
                size: 100,
                layout,
            }
        );

        assert_eq!(
            layout.page_at(100).unwrap(),
            PageInfo {
                number: 1,
                offset_in_page: 0,
                size: 100,
                layout,
            }
        );

        assert_eq!(
            layout.page_at(150).unwrap(),
            PageInfo {
                number: 1,
                offset_in_page: 50,
                size: 100,
                layout,
            }
        );

        assert_eq!(
            layout.page_at(199).unwrap(),
            PageInfo {
                number: 1,
                offset_in_page: 99,
                size: 100,
                layout,
            }
        );

        assert_eq!(
            layout.page_at(300).unwrap(),
            PageInfo {
                number: 3,
                offset_in_page: 0,
                size: 28,
                layout,
            }
        );

        assert_eq!(
            layout.page_at(328).unwrap(),
            PageInfo {
                number: 3,
                offset_in_page: 28,
                size: 28,
                layout,
            }
        );
    }
}
