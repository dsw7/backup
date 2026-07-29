use std::env;
use std::io;
use std::path::PathBuf;

fn get_home_dir() -> io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Couldn't get home directory",
        )),
    }
}

pub fn get_data_dir() -> io::Result<PathBuf> {
    let home_dir = get_home_dir()?;
    let program_dir = home_dir.join(".backup");

    if program_dir.exists() {
        Ok(program_dir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Program data directory does not exist",
        ))
    }
}
