use std::process::{Command, ExitStatus};

use crate::configs::Config;
use crate::errors::BackupError;

fn append_slash_to_source(source: &String) -> String {
    if source.ends_with('/') {
        String::from(source)
    } else {
        String::from(format!("{source}/"))
    }
}

fn remove_slash_from_destination(destination: &String) -> String {
    if destination.ends_with('/') {
        String::from(&destination[..destination.len() - 1])
    } else {
        String::from(destination)
    }
}

fn format_destination(user: &String, host: &String, destination: &String) -> String {
    let dst = remove_slash_from_destination(destination);
    format!("{user}@{host}:{dst}")
}

fn select_destination(sync_to_hot: bool, configs: &Config) -> String {
    if sync_to_hot {
        format_destination(
            &configs.storage.hot.user,
            &configs.storage.hot.host,
            &configs.storage.hot.destination,
        )
    } else {
        format_destination(
            &configs.storage.cold.user,
            &configs.storage.cold.host,
            &configs.storage.cold.destination,
        )
    }
}

fn check_exit_status(exit_status: &ExitStatus) -> Result<String, BackupError> {
    if exit_status.success() {
        return Ok(String::from("Success!"));
    }

    match exit_status.code() {
        Some(code) => Err(BackupError(format!("Subprocess exited with code {code}"))),
        None => Err(BackupError(String::from("Subprocess terminated by signal"))),
    }
}

fn run_rsync_dry_run(src: &String, dst: &String) -> Result<String, BackupError> {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--dry-run")
        .arg("--delete")
        .arg(src)
        .arg(dst)
        .status()
        .expect("Command failed to start. There is no way to proceed");

    check_exit_status(&status)
}

pub fn run_backup(configs: &Config) -> Result<String, BackupError> {
    let sync_to_hot = true;

    let src = append_slash_to_source(&configs.source);
    let dst = select_destination(sync_to_hot, &configs);
    run_rsync_dry_run(&src, &dst)
}
