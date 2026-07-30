use crate::errors::BackupError;

use std::process::Command;

pub fn run_rsync_subprocess(
    src: &str,
    user: &String,
    host: &String,
    dst: &String,
) -> Result<(), BackupError> {
    let dst = format!("{user}@{host}:{dst}");

    let status = Command::new("rsync")
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
