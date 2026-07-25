use serde::Deserialize;

use crate::errors::ConfigError;

use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct General {
    pub source: String,
}

#[derive(Deserialize, Debug)]
pub struct HotStorage {
    pub user: String,
    pub host: String,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct ColdStorage {
    pub user: String,
    pub host: String,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub storage_hot: HotStorage,
    pub storage_cold: ColdStorage,
}

fn get_home_dir() -> Result<PathBuf, ConfigError> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => return Err(ConfigError(String::from("Couldn't get home directory"))),
    }
}

fn get_config_file_path(home_dir: &PathBuf) -> PathBuf {
    let mut config_file_path = PathBuf::from(home_dir);
    config_file_path.push(".backup/config.toml");
    config_file_path
}

fn read_config_file_to_toml_string(config_file_path: &PathBuf) -> Result<String, ConfigError> {
    match fs::read_to_string(config_file_path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            return Err(ConfigError(format!(
                "Failed to read '{}': {e}",
                config_file_path.display()
            )));
        }
    }
}

fn parse_toml_string(toml_str: &String) -> Result<Config, ConfigError> {
    match toml::from_str::<Config>(toml_str) {
        Ok(config) => Ok(config),
        Err(e) => return Err(ConfigError(format!("Failed to parse TOML: {e}"))),
    }
}

pub fn load_configs() -> Result<Config, ConfigError> {
    let home_dir = get_home_dir()?;
    let config_file = get_config_file_path(&home_dir);
    let toml_str = read_config_file_to_toml_string(&config_file)?;
    let configs = parse_toml_string(&toml_str)?;
    Ok(configs)
}
