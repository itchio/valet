pub use reqwest::{
    Client as ReqwestClient, ClientBuilder, Error, IntoUrl, Method, Proxy, Request, RequestBuilder,
    Response,
};
use std::{fmt, time::Duration};
use with_backoff::{futures::BackoffExt, Error as BackoffError, ExponentialBackoff};

pub struct Client {
    pub inner: ReqwestClient,
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
    pub fn new_with_setup<F>(setup: F) -> Result<Self, Error>
    where
        F: FnOnce(ClientBuilder) -> ClientBuilder,
    {
        let mut inner = ReqwestClient::builder()
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

        let inner = setup(inner).build()?;
        Ok(Self { inner })
    }

    pub fn new() -> Result<Self, Error> {
        Self::new_with_setup(|b| b)
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.inner.request(method, url)
    }

    pub async fn execute_no_retry(&self, req: Request) -> Result<Response, Error> {
        self.inner.execute(req.try_clone().unwrap()).await
    }

    pub async fn execute(&self, req: Request) -> Result<Response, Error> {
        let exec = || async {
            self.execute_no_retry(req.try_clone().unwrap())
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
                        BackoffError::Transient(e)
                    } else {
                        BackoffError::Permanent(e)
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

impl<E> FromBackoff<E> for BackoffError<E> {
    fn from_backoff(self) -> E {
        match self {
            Self::Transient(e) | Self::Permanent(e) => e,
        }
    }
}
