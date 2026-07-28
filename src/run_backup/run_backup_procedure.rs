use crate::configs::Configs;
use crate::errors::BackupError;

use super::format_args;
use super::subprocesses;

use std::io::{self, Write};
use std::str::FromStr;

fn read_option_from_stdin() -> io::Result<i32> {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let option = match i32::from_str(input.trim()) {
        Ok(val) => val,
        Err(_) => 0,
    };

    Ok(option)
}

pub fn run_backup_procedure(configs: &Configs) -> Result<(), BackupError> {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[*] -> Exit program");

    let option = read_option_from_stdin()?;

    if option < 1 || option > 4 {
        println!("Backup was manually aborted");
        return Ok(());
    }

    let sync_to_hot = matches!(option, 1 | 2);
    let is_dry_run = matches!(option, 2 | 4);

    let src = format_args::format_src(&configs.source);

    let dst = if sync_to_hot {
        format_args::format_dst_hot(&configs)
    } else {
        format_args::format_dst_cold(&configs)
    };

    if is_dry_run {
        subprocesses::run_rsync_dry_run(&src, &dst)
    } else {
        let log_file = format_args::select_log_file(sync_to_hot)?;
        subprocesses::run_rsync(&src, &dst, &log_file)
    }
}
