use futures_retry::{FutureRetry, RetryPolicy};
use reqwest::{IntoUrl, Method, Request, RequestBuilder};
use std::{fmt, sync::Arc, time::Duration};

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

    pub async fn execute(
        self: Arc<Self>,
        req: Request,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let exec = move || self.inner.execute(req.try_clone().unwrap());

        let mut tries: i32 = 0;
        let (res, _) = FutureRetry::new(exec, |e: reqwest::Error| {
            if e.is_builder() || e.is_redirect() {
                if tries > 1 {
                    tries -= 1;
                    return RetryPolicy::WaitRetry(Duration::from_millis(200));
                }
            }
            return RetryPolicy::ForwardError(e);
        })
        .await
        .map_err(|(e, _attempts)| e)?;
        Ok(res)
    }
}
