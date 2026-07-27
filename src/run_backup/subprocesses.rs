use crate::errors::BackupError;

use std::path::PathBuf;
use std::process::{Command, ExitStatus};

fn check_exit_status(exit_status: &ExitStatus) -> Result<String, BackupError> {
    if exit_status.success() {
        return Ok(String::from("Success!"));
    }

    match exit_status.code() {
        Some(code) => Err(BackupError::SubprocessError(code)),
        None => Err(BackupError::Other(String::from(
            "Subprocess terminated by signal",
        ))),
    }
}

pub fn run_rsync(src: &String, dst: &String, log_file: &PathBuf) -> Result<String, BackupError> {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg(format!("--log-file={}", log_file.display()))
        .arg(src)
        .arg(dst)
        .status()
        .expect("Command failed to start. There is no way to proceed");

    check_exit_status(&status)
}

pub fn run_rsync_dry_run(src: &String, dst: &String) -> Result<String, BackupError> {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg("--dry-run")
        .arg(src)
        .arg(dst)
        .status()
        .expect("Command failed to start. There is no way to proceed");

    check_exit_status(&status)
}
