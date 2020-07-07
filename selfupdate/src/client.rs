use backoff::ExponentialBackoff;
use backoff_futures::BackoffExt;
use reqwest::{IntoUrl, Method, Request, RequestBuilder};
use std::{fmt, time::Duration};

pub struct Client {
    pub inner: reqwest::Client,
}

#[derive(Debug)]
struct UnretriableRequest;

impl fmt::Display for UnretriableRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "request cannot be retried")
    }
}

const USER_AGENT: &str = "valet self-updater";

impl Client {
    pub fn new() -> Result<Self, reqwest::Error> {
        let inner = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self { inner })
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.inner.request(method, url)
    }

    pub async fn execute(&self, req: Request) -> Result<reqwest::Response, reqwest::Error> {
        let exec = || async {
            self.inner
                .execute(req.try_clone().unwrap())
                .await
                .and_then(|r| r.error_for_status())
                .map_err(|e| {
                    let transient = if e.is_status() {
                        e.status().unwrap().is_server_error()
                    } else if e.is_timeout() {
                        true
                    } else {
                        false
                    };
                    if transient {
                        backoff::Error::Transient(e)
                    } else {
                        backoff::Error::Permanent(e)
                    }
                })
        };

        let mut backoff = ExponentialBackoff::default();
        exec.with_backoff(&mut backoff)
            .await
            .map_err(FromBackoff::from_backoff)
    }
}

trait FromBackoff<E> {
    fn from_backoff(self) -> E;
}

impl<E> FromBackoff<E> for backoff::Error<E> {
    fn from_backoff(self) -> E {
        match self {
            Self::Transient(e) | Self::Permanent(e) => e,
        }
    }
}
