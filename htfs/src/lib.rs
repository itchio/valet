#![allow(dead_code)]
#![allow(unused_variables)]

use async_stream::stream;
use bytes::Bytes;
use color_eyre::Report;
use futures::io::AsyncRead;
use futures::stream::TryStreamExt;
use futures_timer::Delay;
use std::{collections::HashMap, fmt, sync::Arc, time::Duration};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sample error")]
    Sample,
    #[error("Trying to get reader at {file_end} but file ends at {requested}")]
    ReadAfterEnd { file_end: u64, requested: u64 },
}

pub struct File {
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
    pub async fn new(url: Url) -> Result<Arc<Self>, Error> {
        tokio::spawn(async move {
            log::debug!("in File task...");
            for _ in 0..5_i32 {
                Delay::new(Duration::from_millis(250)).await;
                log::debug!("in File task loop...");
            }

            let res: Result<(), ()> = Ok(());
            res
        });

        let f = Self {
            url,
            size: 0,
            blocks: Default::default(),
        };
        Ok(Arc::new(f))
    }

    #[tracing::instrument]
    pub async fn get_reader(self: Arc<Self>, offset: u64) -> Result<impl AsyncRead, Report> {
        if offset > self.size {
            Err(Error::ReadAfterEnd {
                file_end: self.size,
                requested: offset,
            })?
        } else {
            let stream = stream! {
                for _ in 0..5_u32 {
                    yield Ok(Bytes::from("hello world"));
                }
            };

            Ok(Box::pin(stream).into_async_read())
        }
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

        let fmt_layer = fmt::layer().with_target(false);
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
        std::env::set_var("RUST_LOG", "debug");
        install_tracing();
        color_eyre::install().unwrap();
        some_test_inner().await.unwrap();
    }

    #[tracing::instrument]
    async fn some_test_inner() -> Result<(), Report> {
        let u = "https://example.org/".parse().unwrap();
        let f = File::new(u).await?;

        let mut reader = f.get_reader(0).await?;
        let mut buf = vec![0u8; 16];
        loop {
            let n = reader.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            log::info!("read {} bytes", n);
            log::info!("{:?}", String::from_utf8_lossy(&buf[..n]));
        }

        Ok(())
    }
}
