mod client;
mod platform;

use client::Client;
use reqwest::Method;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("dummy error")]
    DummyError,
    #[error("request error: {0}")]
    RequestError(String),
    #[error("platform error: {0}")]
    PlatformError(#[from] platform::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}

pub struct Settings {
    pub components_dir: PathBuf,
    pub is_canary: bool,
}

const BROTH_BASE_URL: &str = "https://broth.itch.ovh";

#[derive(Deserialize, Debug)]
struct VersionsList {
    versions: Vec<String>,
}

pub async fn check(settings: &Settings) -> Result<String, Error> {
    log::info!("Checking for update...");

    let channel = platform::get_channel_name(settings)?;
    log::info!("For channel {}", channel);
    let channel_url = format!("{}/itch-setup/{}", BROTH_BASE_URL, channel);

    let client = Arc::new(Client::new()?);
    let version_url = format!("{}/versions", channel_url);
    let req = client.request(Method::GET, &version_url).build()?;

    let versions_list: VersionsList = client
        .clone()
        .execute(req)
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?
        .json()
        .await?;

    let latest_version = versions_list.versions.first();

    let res = format!("latest version of itch-setup is {:?}", latest_version);
    Ok(res)
}
