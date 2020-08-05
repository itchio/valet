use async_trait::async_trait;
use color_eyre::Report;
use futures::lock::Mutex;
use reqwest::Method;
use std::{fmt, sync::Arc};
use url::Url;

mod reader;
use reader::ResourceReader;
mod conn;
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
    #[error("zero-length resource: the content-length header was not present or zero")]
    ZeroLength,
    #[error("trying to get reader at {requested} but resource ends at {resource_end}")]
    ReadAfterEnd { resource_end: u64, requested: u64 },
}

pub struct Resource {
    inner: Arc<ResourceInner>,
}

struct ResourceInner {
    client: reqwest::Client,
    url: Url,
    size: u64,
    connections: Mutex<Vec<Conn>>,
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "htfs::File({:?})", self.inner.url)
    }
}

impl Resource {
    #[tracing::instrument]
    pub async fn new(url: Url) -> Result<Self, Report> {
        let client = reqwest::Client::new();

        let mut inner = ResourceInner {
            client,
            url,
            size: 0,
            connections: Default::default(),
        };

        let conn = inner.borrow_conn(0).await?;
        let connections: Mutex<Vec<Conn>> = Default::default();

        if let Some(header) = conn.headers.get("content-length") {
            if let Ok(header) = header.to_str() {
                if let Ok(size) = header.parse() {
                    inner.size = size;
                }
            }
        };

        if inner.size == 0 {
            return Err(Error::ZeroLength)?;
        }

        {
            let mut connections = connections.lock().await;
            connections.push(conn);
        }

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn size(&self) -> u64 {
        self.inner.size
    }
}

impl ResourceInner {
    pub(crate) async fn borrow_conn(&self, offset: u64) -> Result<Conn, Report> {
        let mut conns = self.connections.lock().await;
        let candidate =
            conns.iter().enumerate().find_map(
                |(i, conn)| {
                    if conn.offset == offset {
                        Some(i)
                    } else {
                        None
                    }
                },
            );

        match candidate {
            Some(index) => {
                let conn = conns.remove(index);
                tracing::debug!("{:?}: re-using", conn);
                Ok(conn)
            }
            None => {
                drop(conns);
                Ok(self.new_conn(offset).await?)
            }
        }
    }

    async fn new_conn(&self, offset: u64) -> Result<Conn, Report> {
        let range = format!("bytes={}-", offset);
        let req = self
            .client
            .request(Method::GET, self.url.clone())
            .header("range", range)
            .build()?;
        let res = self.client.execute(req).await?;
        let conn = Conn::new(res, offset);
        tracing::debug!("{:?}", conn);
        Ok(conn)
    }

    pub(crate) async fn return_conn(&self, conn: Conn) {
        // TODO: expire old conns
        let mut connections = self.connections.lock().await;
        connections.push(conn)
    }
}

#[async_trait(?Send)]
impl GetReaderAt for Resource {
    type Reader = ResourceReader;

    async fn get_reader_at(&self, offset: u64) -> std::io::Result<Self::Reader> {
        if offset > self.inner.size {
            Err(make_io_error(Error::ReadAfterEnd {
                resource_end: self.inner.size,
                requested: offset,
            }))?
        } else {
            Ok(ResourceReader::new(self.inner.clone(), offset))
        }
    }

    fn size(&self) -> u64 {
        self.inner.size
    }
}
