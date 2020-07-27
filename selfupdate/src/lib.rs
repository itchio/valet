pub mod broth;
pub mod platform;

use httpkit::Client;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("dummy error")]
    DummyError,
    #[error("broth error: {0}")]
    BrothError(#[from] broth::Error),
    #[error("platform error: {0}")]
    PlatformError(#[from] platform::Error),
    #[error("httpkit error: {0}")]
    HttpKitError(#[from] httpkit::Error),
    #[error("no latest version of itch-setup")]
    NoLatestVersion,
}

pub struct Settings {
    pub components_dir: PathBuf,
    pub is_canary: bool,
}

pub async fn check(settings: &Settings) -> Result<String, Error> {
    log::info!("Checking for itch-setup updates...");

    let spec = broth::ComponentSpec {
        settings: &broth::Settings {
            components_dir: settings.components_dir.clone(),
        },
        component: "itch-setup".into(),
        channel: platform::get_channel_name(settings)?,
    };

    let client = Client::new()?;

    let latest_version = spec
        .get_latest_version(&client)
        .await?
        .ok_or(Error::NoLatestVersion)?;
    let present_versions = spec.get_present_versions()?;

    log::info!("found {} present versions: ", present_versions.len());
    for v in &present_versions {
        log::info!("- {}", v);
    }
    log::info!("latest version: {}", latest_version);

    Ok("stub!".into())
}
