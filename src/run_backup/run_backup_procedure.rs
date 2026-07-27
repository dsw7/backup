use crate::configs::Configs;
use crate::data_directory;
use crate::errors::BackupError;

use super::format_args;
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

fn select_log_file(sync_to_hot: bool) -> io::Result<PathBuf> {
    let program_dir = data_directory::get_data_dir()?;

    if sync_to_hot {
        Ok(PathBuf::from(program_dir).join("backup_hot.log"))
    } else {
        Ok(PathBuf::from(program_dir).join("backup_cold.log"))
    }
}

pub fn run_backup_procedure(configs: &Configs) -> Result<String, BackupError> {
    let (sync_to_hot, is_dry_run, exit_program) = select_backup_type();

    if exit_program {
        return Ok(String::from("Program manually aborted"));
    }

    let src = format_args::format_src(&configs.source);

    let dst = if sync_to_hot {
        format_args::format_dst_hot(&configs)
    } else {
        format_args::format_dst_cold(&configs)
    };

    if is_dry_run {
        subprocesses::run_rsync_dry_run(&src, &dst)
    } else {
        let log_file = select_log_file(sync_to_hot)?;
        subprocesses::run_rsync(&src, &dst, &log_file)
    }
}
