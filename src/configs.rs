use serde::Deserialize;

use crate::data_directory;
use crate::errors::BackupError;

use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Configs {
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
    pub user: String,
    pub host: String,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct Cold {
    pub user: String,
    pub host: String,
    pub destination: String,
}

fn get_config_file_path() -> Result<PathBuf, BackupError> {
    let program_dir = data_directory::get_data_dir()?;
    Ok(PathBuf::from(program_dir).join("config.toml"))
}

fn read_config_file_to_toml_string(config_file: &PathBuf) -> Result<String, BackupError> {
    match fs::read_to_string(config_file) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(BackupError::ConfigurationError(e.to_string())),
    }
}

fn parse_toml_string(toml_str: &String) -> Result<Configs, BackupError> {
    match toml::from_str::<Configs>(toml_str) {
        Ok(config) => Ok(config),
        Err(e) => Err(BackupError::ConfigurationError(e.to_string())),
    }
}

pub fn load_configs() -> Result<Configs, BackupError> {
    let config_file = get_config_file_path()?;
    let toml_str = read_config_file_to_toml_string(&config_file)?;
    let configs = parse_toml_string(&toml_str)?;
    Ok(configs)
}
