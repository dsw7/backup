use crate::configs::Config;
use crate::errors::BackupError;
use crate::program_files::get_program_data_dir;

use super::subprocesses;

use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

fn read_option_from_stdin() -> i32 {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Unrecoverable error: Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unrecoverable error: Failed to read from stdin");

    match i32::from_str(input.trim()) {
        Ok(val) => val,
        Err(_) => 0,
    }
}

fn select_backup_type() -> (bool, bool, bool) {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[*] -> Exit program");

    let option = read_option_from_stdin();

    let sync_to_hot = matches!(option, 1 | 2);
    let is_dry_run = matches!(option, 2 | 4);
    let exit_program = option < 1 || option > 4;

    (sync_to_hot, is_dry_run, exit_program)
}

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
    let (sync_to_hot, is_dry_run, exit_program) = select_backup_type();

    if exit_program {
        return Ok(String::from("Program manually aborted"));
    }

    let src = append_slash_to_source(&configs.source);
    let dst = select_destination(sync_to_hot, &configs);

    if is_dry_run {
        subprocesses::run_rsync_dry_run(&src, &dst)
    } else {
        let log_file = select_log_file(sync_to_hot)?;
        subprocesses::run_rsync(&src, &dst, &log_file)
    }
}
