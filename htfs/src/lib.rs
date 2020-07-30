use futures_io::AsyncRead;
use futures_timer::Delay;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sample error")]
    Sample,
    #[error("end of file reached")]
    EOF,
}

pub struct File {
    url: Url,
    size: u64,
    blocks: HashMap<u64, Vec<u8>>,
}

impl File {
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

    pub fn get_reader(self: Arc<Self>, offset: u64) -> Result<impl AsyncRead, Error> {
        if offset > self.size {
            Ok(Reader { file: self, offset })
        } else {
            Err(Error::EOF)
        }
    }
}

struct Reader {
    file: Arc<File>,
    offset: u64,
}

impl AsyncRead for Reader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<futures_io::Result<usize>> {
        log::debug!("Should read at offset {}", self.offset);

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(threaded_scheduler)]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        std::env::set_var("RUST_LOG", "debug");
        pretty_env_logger::init();

        let u = "https://example.org/".parse().unwrap();
        let f = File::new(u).await?;
        std::thread::sleep(Duration::from_secs(2));

        Ok(())
    }
}
