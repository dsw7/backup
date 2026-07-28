use crate::errors::BackupError;

use std::path::PathBuf;
use std::process::Command;

fn append_slash_to_src(src: &String) -> String {
    if src.ends_with('/') {
        String::from(src)
    } else {
        String::from(format!("{src}/"))
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
    log_file: &PathBuf,
) -> Result<(), BackupError> {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg(format!("--log-file={}", log_file.display()))
        .arg(append_slash_to_src(&src))
        .arg(format!("{user}@{host}:{}", remove_slash_from_dst(&dst)))
        .status()?;

    if !status.success() {
        eprintln!("Synchronization failed");
    }

    Ok(())
}

pub fn run_rsync_dry_run(
    src: &String,
    user: &String,
    host: &String,
    dst: &String,
) -> Result<(), BackupError> {
    let status = Command::new("rsync2")
        .arg("-av")
        .arg("--delete")
        .arg("--dry-run")
        .arg(append_slash_to_src(&src))
        .arg(format!("{user}@{host}:{}", remove_slash_from_dst(&dst)))
        .status()?;

    if !status.success() {
        eprintln!("Synchronization failed");
    }

    Ok(())
}
