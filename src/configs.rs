use serde::Deserialize;
use std::env;
use std::fs;
use std::io;
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

fn get_home_dir() -> Result<PathBuf, io::Error> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Couldn't get home directory",
            ));
        }
    }
}

fn get_config_file_path(home_dir: &PathBuf) -> Result<PathBuf, io::Error> {
    let mut config_file_path = PathBuf::from(home_dir);
    config_file_path.push(".backup/config.toml");

    Ok(config_file_path)
}

fn read_config_file_to_string(config_file_path: &PathBuf) -> Result<String, io::Error> {
    match fs::read_to_string(config_file_path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read '{}': {e}", config_file_path.display()),
            ));
        }
    }
}

pub fn load_configs() -> Result<Config, io::Error> {
    let home_dir = get_home_dir()?;
    let config_file = get_config_file_path(&home_dir)?;
    let contents = read_config_file_to_string(&config_file)?;

    let config: Config = toml::from_str(&contents).expect("Failed to parse TOML content");
    Ok(config)
}
