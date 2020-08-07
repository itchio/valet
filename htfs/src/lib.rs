use async_trait::async_trait;
use color_eyre::eyre;
use futures::{AsyncRead, TryStreamExt};
use reqwest::Method;
use std::{fmt, sync::Arc};
use url::Url;

use ara::{
    buf_reader_at::BufReaderAt,
    read_at_wrapper::{GetReaderAt, ReadAtWrapper},
    ReadAt,
};
use errors::make_io_error;
pub(crate) mod errors;
mod rand_id;

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("zero-length resource: the content-length header was not present or zero")]
    ZeroLength,
    #[error("trying to get reader at {requested} but resource ends at {resource_end}")]
    ReadAfterEnd { resource_end: u64, requested: u64 },
}

pub struct Resource {
    client: reqwest::Client,
    url: Url,
    size: u64,
    initial_response: Option<reqwest::Response>,
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "htfs::File({:?})", self.url)
    }
}

impl Resource {
    #[tracing::instrument]
    pub async fn new(url: Url) -> Result<Self, eyre::Error> {
        let client = reqwest::Client::new();

        let mut resource = Resource {
            client,
            url,
            size: 0,
            initial_response: None,
        };

        let res = resource.request(0).await?;
        // TODO: switch back to res.content_length()
        if let Some(header) = res.headers().get("content-length") {
            if let Ok(header) = header.to_str() {
                if let Ok(size) = header.parse() {
                    resource.size = size;
                }
            }
        };

        if resource.size == 0 {
            return Err(Error::ZeroLength)?;
        }
        resource.initial_response = resource.initial_response;

        Ok(resource)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn into_read_at(mut self) -> impl ReadAt {
        let initial_response =
            self.initial_response
                .take()
                .map(|res| -> (u64, Box<dyn AsyncRead + Unpin>) {
                    let reader =
                        Box::new(res.bytes_stream().map_err(make_io_error).into_async_read());
                    (0, reader)
                });
        let size = self.size;
        let source = Arc::new(self);

        let r = ReadAtWrapper::new(source, size, initial_response);
        let r = BufReaderAt::new(r);
        r
    }

    async fn request(&self, offset: u64) -> Result<reqwest::Response, eyre::Error> {
        let range = format!("bytes={}-", offset);
        let req = self
            .client
            .request(Method::GET, self.url.clone())
            .header("range", range)
            .build()?;
        let res = self.client.execute(req).await?;
        Ok(res)
    }
}

#[async_trait(?Send)]
impl GetReaderAt for Resource {
    type Reader = Box<dyn AsyncRead + Unpin>;

    async fn get_reader_at(self: &Arc<Self>, offset: u64) -> std::io::Result<Self::Reader> {
        if offset > self.size {
            Err(make_io_error(Error::ReadAfterEnd {
                resource_end: self.size,
                requested: offset,
            }))?
        } else {
            let res = self.request(offset).await.map_err(make_io_error)?;
            Ok(Box::new(
                res.bytes_stream().map_err(make_io_error).into_async_read(),
            ))
        }
    }
}
