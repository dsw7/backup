use std::process::Command;

use anyhow::Context;

pub fn run_rsync_subprocess(src: &str, user: &str, host: &str, dst: &str) -> anyhow::Result<()> {
    let dst = format!("{user}@{host}:{dst}");

    let status = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg("--dry-run")
        .arg(src)
        .arg(dst)
        .status()
        .context("Failed to run `rsync` subprocess")?;

    if !status.success() {
        eprintln!("The dry run procedure failed");
    }

    Ok(())
}
