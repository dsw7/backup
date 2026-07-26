use crate::configs::Config;
use crate::errors::BackupError;
use crate::program_files::get_program_data_dir;

use super::get_backup_options::select_backup_type;
use super::subprocesses::{run_rsync, run_rsync_dry_run};

use std::path::PathBuf;

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

fn select_log_file(sync_to_hot: bool) -> Result<PathBuf, BackupError> {
    let program_dir = get_program_data_dir()?;

    if sync_to_hot {
        Ok(PathBuf::from(program_dir).join("backup_hot.log"))
    } else {
        Ok(PathBuf::from(program_dir).join("backup_cold.log"))
    }
}

pub fn run_backup(configs: &Config) -> Result<String, BackupError> {
    let backup_options = select_backup_type();

    if backup_options.exit_program {
        return Ok(String::from("Program manually aborted"));
    }

    let src = append_slash_to_source(&configs.source);
    let dst = select_destination(backup_options.sync_to_hot, &configs);

    if backup_options.is_dry_run {
        run_rsync_dry_run(&src, &dst)
    } else {
        let log_file = select_log_file(backup_options.sync_to_hot)?;
        run_rsync(&src, &dst, &log_file)
    }
}
