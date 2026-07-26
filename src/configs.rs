use serde::Deserialize;

use crate::errors::BackupError;
use crate::program_files::get_config_file_path;

use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
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

fn read_config_file_to_toml_string(config_path: &PathBuf) -> Result<String, BackupError> {
    match fs::read_to_string(config_path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(BackupError(format!(
            "Failed to read '{}': {e}",
            config_path.display()
        ))),
    }
}

fn parse_toml_string(toml_str: &String) -> Result<Config, BackupError> {
    match toml::from_str::<Config>(toml_str) {
        Ok(config) => Ok(config),
        Err(e) => Err(BackupError(format!("Failed to parse TOML: {e}"))),
    }
}

pub fn load_configs() -> Result<Config, BackupError> {
    let config_file = get_config_file_path()?;
    let toml_str = read_config_file_to_toml_string(&config_file)?;
    let configs = parse_toml_string(&toml_str)?;
    Ok(configs)
}
