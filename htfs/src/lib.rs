#![allow(dead_code)]
#![allow(unused_variables)]

use async_stream::stream;
use color_eyre::Report;
use futures::io::AsyncRead;
use futures::prelude::*;
use futures::stream::TryStreamExt;
use reqwest::Method;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};
use url::Url;

mod reader;
use reader::Reader;

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
    blocks: HashMap<u64, Vec<u8>>,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "htfs::File({:?})", self.url)
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

        let f = Self {
            client,
            url,
            size,
            blocks: Default::default(),
        };
        Ok(f)
    }

    pub async fn get_reader(&self, offset: u64) -> Result<impl AsyncRead, Report> {
        if offset > self.size {
            Err(Error::ReadAfterEnd {
                file_end: self.size,
                requested: offset,
            })?
        } else {
            let req = self
                .client
                .request(Method::GET, self.url.clone())
                .header("range", format!("bytes={}-", offset))
                .build()?;
            let res = self.client.execute(req).await?;
            let mut body = res.bytes_stream();

            let stream = stream! {
                while let Some(chunk) = body.next().await {
                    match chunk {
                        Ok(chunk) => {
                            yield Ok(chunk)
                        },
                        Err(e) => {
                            yield Err(std::io::Error::new(std::io::ErrorKind::Other, e))
                        }
                    }
                }
            };
            let reader = Box::pin(stream).into_async_read();
            let reader = Reader {
                reader: Arc::new(Mutex::new(reader)),
                fut: None,
            };

            Ok(reader)
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::AsyncReadExt;

    fn install_tracing() {
        use tracing_error::ErrorLayer;
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{fmt, EnvFilter};

        let fmt_layer = fmt::layer();
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(ErrorLayer::default())
            .init();
    }

    #[tokio::test(threaded_scheduler)]
    async fn some_test() {
        std::env::set_var("RUST_LOG", "reqwest=debug,hyper::client=debug,htfs=debug");
        install_tracing();
        color_eyre::install().unwrap();
        some_test_inner().await.unwrap();
    }

    #[tracing::instrument]
    async fn some_test_inner() -> Result<(), Report> {
        let u = "https://example.org/".parse().unwrap();
        let f = File::new(u).await?;

        let mut buf = vec![0u8; 29];
        let mut reader = f.get_reader(34).await?;
        reader.read_exact(&mut buf).await?;

        log::info!("{:?}", String::from_utf8_lossy(&buf[..]));

        Ok(())
    }
}
