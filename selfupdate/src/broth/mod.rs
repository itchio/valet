use httpkit::{Client, Method};
use serde::Deserialize;
use std::path::PathBuf;

const BROTH_BASE_URL: &str = "https://broth.itch.ovh";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("httpkit error: {0}")]
    HttpKitError(#[from] httpkit::Error),
    #[error("i/o error: {0}")]
    IOError(#[from] std::io::Error),
}

pub struct Settings {
    pub components_dir: PathBuf,
}

pub struct ComponentSpec<'a> {
    pub settings: &'a Settings,
    pub component: String,
    pub channel: String,
}

impl ComponentSpec<'_> {
    pub fn channel_url(&self) -> String {
        format!("{}/{}/{}", BROTH_BASE_URL, self.component, self.channel)
    }

    pub async fn get_latest_version(&self, client: &Client) -> Result<Option<String>, Error> {
        let version_url = format!("{}/versions", self.channel_url());
        let req = client.request(Method::GET, &version_url).build()?;

        let versions_list: VersionsList = client.execute(req).await?.json().await?;
        Ok(versions_list.versions.into_iter().last())
    }

    pub fn get_present_versions(&self) -> Result<Vec<String>, Error> {
        let versions_dir = self
            .settings
            .components_dir
            .join(&self.component)
            .join("versions");
        log::debug!(
            "Looking for {} versions in {}",
            self.component,
            versions_dir.display()
        );

        let mut res: Vec<String> = Default::default();

        if !versions_dir.exists() {
            std::fs::create_dir_all(&versions_dir)?;
        }

        for entry in std::fs::read_dir(&versions_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(name) = entry.path().file_name() {
                    res.push(name.to_string_lossy().into())
                }
            }
        }

        Ok(res)
    }
}

#[derive(Deserialize, Debug)]
pub struct VersionsList {
    pub versions: Vec<String>,
}
