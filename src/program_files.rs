use crate::errors::BackupError;

use std::env;
use std::path::PathBuf;

fn get_home_dir() -> Result<PathBuf, BackupError> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => return Err(BackupError(String::from("Couldn't get home directory"))),
    }
}

fn get_program_data_dir(home_dir: &PathBuf) -> Result<PathBuf, BackupError> {
    let mut program_dir = PathBuf::from(home_dir);
    program_dir.push(".backup");

    if program_dir.exists() {
        Ok(program_dir)
    } else {
        Err(BackupError(format!(
            "Directory '{}' does not exist",
            program_dir.display()
        )))
    }
}

pub fn get_config_file_path() -> Result<PathBuf, BackupError> {
    let home_dir = get_home_dir()?;
    let program_dir = get_program_data_dir(&home_dir)?;

    let mut config_file_path = PathBuf::from(program_dir);
    config_file_path.push("config.toml");

    Ok(config_file_path)
}
