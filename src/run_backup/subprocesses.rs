use crate::errors::BackupError;

use std::path::PathBuf;
use std::process::Command;

pub fn run_rsync(src: &String, dst: &String, log_file: &PathBuf) -> Result<(), BackupError> {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg(format!("--log-file={}", log_file.display()))
        .arg(src)
        .arg(dst)
        .status()?;

    if !status.success() {
        eprintln!("Synchronization failed");
    }
    Ok(())
}

pub fn run_rsync_dry_run(src: &String, dst: &String) -> Result<(), BackupError> {
    let status = Command::new("rsync2")
        .arg("-av")
        .arg("--delete")
        .arg("--dry-run")
        .arg(src)
        .arg(dst)
        .status()?;

    if !status.success() {
        eprintln!("Synchronization failed");
    }
    Ok(())
}
