use serde::{Deserialize, Deserializer};

use crate::data_directory;
use crate::errors::BackupError;

use std::fs;
use std::path::PathBuf;

fn check_not_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    if value.is_empty() {
        Err(serde::de::Error::custom("String cannot be empty"))
    } else {
        Ok(value)
    }
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    #[serde(deserialize_with = "check_not_empty")]
    pub source: String,
    pub storage: Storage,
}

#[derive(Deserialize, Debug)]
pub struct Storage {
    pub hot: Hot,
    pub cold: Cold,
}

#[derive(Deserialize, Debug)]
pub struct Hot {
    #[serde(deserialize_with = "check_not_empty")]
    pub user: String,

    #[serde(deserialize_with = "check_not_empty")]
    pub host: String,

    #[serde(deserialize_with = "check_not_empty")]
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct Cold {
    #[serde(deserialize_with = "check_not_empty")]
    pub user: String,

    #[serde(deserialize_with = "check_not_empty")]
    pub host: String,

    #[serde(deserialize_with = "check_not_empty")]
    pub destination: String,
}

fn get_config_file_path() -> Result<PathBuf, BackupError> {
    let program_dir = data_directory::get_data_dir()?;
    Ok(program_dir.join("config.toml"))
}

pub fn load_configs() -> Result<Configs, BackupError> {
    let config_file = get_config_file_path()?;
    let toml_str = fs::read_to_string(&config_file)?;
    let configs = toml::from_str::<Configs>(&toml_str)?;
    Ok(configs)
}
