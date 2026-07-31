use std::env;
use std::io;
use std::path::{Path, PathBuf};

fn get_home_dir() -> io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Couldn't get home directory",
        )),
    }
}

pub fn get_app_dir() -> io::Result<PathBuf> {
    let home_dir = get_home_dir()?;
    Ok(home_dir.join(".backup"))
}

pub fn get_readme_file(app_dir: &Path) -> PathBuf {
    app_dir.join("README.txt")
}

pub fn get_config_file(app_dir: &Path) -> PathBuf {
    app_dir.join("config.toml")
}
