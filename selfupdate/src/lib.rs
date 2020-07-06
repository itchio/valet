use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("dummy error")]
    DummyError,
    #[error("request error: {0}")]
    RequestError(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}

#[derive(Deserialize, Debug)]
struct Country {
    country: String,
    network: Option<String>,
}

pub async fn check() -> Result<String, Error> {
    log::info!("Checking for update...");

    let resp: Country = reqwest::get("https://itch.io/country")
        .await?
        .json()
        .await?;

    log::info!("Dummy update check complete");
    if rand::random() {
        log::warn!("Failing for fun");
        return Err(Error::DummyError);
    }

    Ok(format!("{:#?}", resp))
}
