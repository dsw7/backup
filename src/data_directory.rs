use crate::errors::BackupError;

use std::env;
use std::path::PathBuf;

fn get_home_dir() -> Result<PathBuf, BackupError> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => Err(BackupError(String::from("Couldn't get home directory"))),
    }
}

pub fn get_data_dir() -> Result<PathBuf, BackupError> {
    let home_dir = get_home_dir()?;
    let program_dir = PathBuf::from(home_dir).join(".backup");

    if program_dir.exists() {
        Ok(program_dir)
    } else {
        Err(BackupError(format!(
            "Directory '{}' does not exist",
            program_dir.display()
        )))
    }
}
