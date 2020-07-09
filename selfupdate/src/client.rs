use backoff::{futures::BackoffExt, ExponentialBackoff};
use reqwest::{IntoUrl, Method, Proxy, Request, RequestBuilder};
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
        let mut inner = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(30));

        if let Ok(http_proxy) = std::env::var("http_proxy") {
            inner = inner.proxy(Proxy::all(&http_proxy)?);
        }

        const IGNORE_CERT_ERRORS_KEY: &str = "HTTPKIT_IGNORE_CERTIFICATE_ERRORS";
        if let Some("1") = std::env::var(IGNORE_CERT_ERRORS_KEY).ok().as_deref() {
            log::warn!(
                "Accepting invalid certificates ({:?} set to 1)",
                IGNORE_CERT_ERRORS_KEY
            );
            inner = inner.danger_accept_invalid_certs(true)
        }

        let inner = inner.build()?;
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

        let backoff = ExponentialBackoff::default();
        exec.with_backoff(backoff)
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
