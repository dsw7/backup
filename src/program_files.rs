use std::env;
use std::path::{Path, PathBuf};

fn get_home_dir() -> anyhow::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => anyhow::bail!("Couldn't get home directory"),
    }
}

pub fn get_app_dir() -> anyhow::Result<PathBuf> {
    let home_dir = get_home_dir()?;
    Ok(home_dir.join(".backup"))
}

pub fn get_readme_file(app_dir: &Path) -> PathBuf {
    app_dir.join("README.txt")
}

pub fn get_config_file(app_dir: &Path) -> PathBuf {
    app_dir.join("config.toml")
}

#[cfg(debug_assertions)]
pub fn get_log_dir(app_dir: &Path) -> PathBuf {
    let log_dir = app_dir.join("logs_debug");

    println!("\n*** Debug build detected!");
    println!("*** Will place log files under: {}\n", log_dir.display());
    log_dir
}

#[cfg(not(debug_assertions))]
pub fn get_log_dir(app_dir: &Path) -> PathBuf {
    app_dir.join("logs")
}
