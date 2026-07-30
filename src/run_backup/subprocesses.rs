use tracing::Level;

use crate::errors::BackupError;

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
}

fn append_slash_to_src(src: &String) -> String {
    if src.ends_with('/') {
        String::from(src)
    } else {
        format!("{src}/")
    }
}

fn remove_slash_from_dst(dst: &String) -> String {
    if dst.ends_with('/') {
        String::from(&dst[..dst.len() - 1])
    } else {
        String::from(dst)
    }
}

pub fn run_rsync(
    src: &String,
    user: &String,
    host: &String,
    dst: &String,
    log_file: &Path,
) -> Result<(), BackupError> {
    init_logger();
    tracing::info!("Starting data synchronization");

    let mut child = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg(format!("--log-file={}", log_file.display()))
        .arg(append_slash_to_src(src))
        .arg(format!("{user}@{host}:{}", remove_slash_from_dst(dst)))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    let handle_stdout = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            match line {
                Ok(text) => tracing::info!("{text}"),
                Err(e) => tracing::error!("Error reading line: {}", e),
            }
        }
    });

    let handle_stderr = thread::spawn(move || {
        for line in BufReader::new(stderr).lines() {
            match line {
                Ok(text) => tracing::error!("{text}"),
                Err(e) => tracing::error!("Error reading line: {}", e),
            }
        }
    });

    handle_stdout.join().unwrap();
    handle_stderr.join().unwrap();

    let status = child.wait()?;

    if status.success() {
        tracing::info!("Synchronization succeeded");
    } else {
        tracing::error!("Synchronization failed");
    }

    Ok(())
}

pub fn run_rsync_dry_run(
    src: &String,
    user: &String,
    host: &String,
    dst: &String,
) -> Result<(), BackupError> {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg("--dry-run")
        .arg(append_slash_to_src(src))
        .arg(format!("{user}@{host}:{}", remove_slash_from_dst(dst)))
        .status()?;

    if !status.success() {
        eprintln!("Synchronization failed");
    }

    Ok(())
}
