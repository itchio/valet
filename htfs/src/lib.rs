#![allow(dead_code)]
#![allow(unused_variables)]

use color_eyre::Report;
use futures::io::AsyncRead;
use futures::lock::Mutex;
use reqwest::Method;
use std::{collections::HashMap, fmt, sync::Arc};
use url::Url;

mod reader;
use reader::Reader2;
mod conn;
mod response_reader;
use conn::Conn;
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
    client: reqwest::Client,
    url: Url,
    size: u64,
    connections: Mutex<Vec<Conn<'static>>>,
    blocks: Mutex<HashMap<u64, Vec<u8>>>,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "htfs::File({:?})", self.url)
    }
}

impl File {
    #[tracing::instrument]
    pub async fn new(url: Url) -> Result<Arc<Self>, Report> {
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

        let f = Self {
            client,
            url,
            size,
            connections,
            blocks: Default::default(),
        };
        Ok(Arc::new(f))
    }

    pub async fn get_reader(self: &Arc<Self>, offset: u64) -> Result<impl AsyncRead, Report> {
        if offset > self.size {
            Err(Error::ReadAfterEnd {
                file_end: self.size,
                requested: offset,
            })?
        } else {
            // let req = self
            //     .client
            //     .request(Method::GET, self.url.clone())
            //     .header("range", format!("bytes={}-", offset))
            //     .build()?;
            // let res = self.client.execute(req).await?;
            // let reader = response_reader::as_reader(res);
            // let reader = Reader2::new(Arc::clone(self), reader);
            // Ok(reader)

            Ok(Reader2::new(Arc::clone(self), offset))
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
