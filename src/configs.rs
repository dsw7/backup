use serde::de::Error as DeserializationError;
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
        Err(DeserializationError::custom("String cannot be empty"))
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

fn read_config_file_to_toml_string(config_file: &PathBuf) -> Result<String, BackupError> {
    match fs::read_to_string(config_file) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(BackupError::ConfigurationError(e.to_string())),
    }
}

fn parse_toml_string(toml_str: &str) -> Result<Configs, BackupError> {
    match toml::from_str::<Configs>(toml_str) {
        Ok(config) => Ok(config),
        Err(e) => Err(BackupError::ConfigurationError(e.to_string())),
    }
}

pub fn load_configs() -> Result<Configs, BackupError> {
    let config_file = get_config_file_path()?;
    let toml_str = read_config_file_to_toml_string(&config_file)?;
    parse_toml_string(&toml_str)
}
