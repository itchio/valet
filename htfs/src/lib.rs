use async_trait::async_trait;
use color_eyre::Report;
use futures::lock::Mutex;
use reqwest::Method;
use std::{fmt, sync::Arc};
use url::Url;

mod reader;
use reader::Reader;
mod conn;
mod response_reader;
use conn::Conn;
pub mod async_read_at;
use async_read_at::*;
use errors::make_io_error;
pub(crate) mod errors;
mod rand_id;

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("zero-length file: the content-length header was not present or zero")]
    ZeroLengthFile,
    #[error("trying to get reader at {requested} but file ends at {file_end}")]
    ReadAfterEnd { file_end: u64, requested: u64 },
}

pub struct File {
    core: Arc<FileCore>,
}

struct FileCore {
    client: reqwest::Client,
    url: Url,
    size: u64,
    connections: Mutex<Vec<Conn<'static>>>,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "htfs::File({:?})", self.core.url)
    }
}

impl File {
    #[tracing::instrument]
    pub async fn new(url: Url) -> Result<Self, Report> {
        let client = reqwest::Client::new();
        let req = client
            .request(Method::GET, url.clone())
            .header("range", "bytes=0-")
            .build()?;
        let res = client.execute(req).await?;
        let size = res.content_length().unwrap_or_default();
        if size == 0 {
            return Err(Error::ZeroLengthFile)?;
        }

        let connections: Mutex<Vec<Conn>> = Default::default();
        {
            let mut connections = connections.lock().await;
            connections.push(Conn::new(response_reader::as_reader(res), 0));
        }

        let core = FileCore {
            client,
            url,
            size,
            connections,
        };
        Ok(Self {
            core: Arc::new(core),
        })
    }

    pub fn size(&self) -> u64 {
        self.core.size
    }
}

#[async_trait(?Send)]
impl GetReaderAt for File {
    type Reader = Reader;

    async fn get_reader_at(&self, offset: u64) -> std::io::Result<Self::Reader> {
        if offset > self.core.size {
            Err(make_io_error(Error::ReadAfterEnd {
                file_end: self.core.size,
                requested: offset,
            }))?
        } else {
            Ok(Reader::new(self.core.clone(), offset))
        }
    }

    fn size(&self) -> u64 {
        self.core.size
    }
}
