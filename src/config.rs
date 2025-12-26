use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub authors: String,
    pub cargo_pkg_version: String,
    pub vendor: String,
    pub vendor_url: String,
    pub vendor_email: String,
    pub nih_plug_git: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let local_config = PathBuf::from("./data/general_info.toml");

        if local_config.exists() {
            let content = fs::read_to_string(&local_config)?;
            Ok(toml::from_str(&content)?)
        } else {
            Self::default_config()
        }
    }

    fn default_config() -> Result<Self> {
        let default_config = include_str!("../data/general_info.toml");

        Ok(toml::from_str(default_config)?)
    }
}
