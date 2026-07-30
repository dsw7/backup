use crate::configs::Configs;
use crate::data_directory::get_data_dir;
use crate::errors::BackupError;

use super::subprocesses;

use std::io::{self, Write};
use std::path::PathBuf;
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

fn append_slash_to_src(src: &String) -> String {
    if src.ends_with('/') {
        String::from(src)
    } else {
        format!("{src}/")
    }
}

fn select_user(sync_to_hot: bool, configs: &Configs) -> &String {
    if sync_to_hot {
        &configs.storage.hot.user
    } else {
        &configs.storage.cold.user
    }
}

fn select_host(sync_to_hot: bool, configs: &Configs) -> &String {
    if sync_to_hot {
        &configs.storage.hot.host
    } else {
        &configs.storage.cold.host
    }
}

fn select_destination(sync_to_hot: bool, configs: &Configs) -> &String {
    if sync_to_hot {
        &configs.storage.hot.destination
    } else {
        &configs.storage.cold.destination
    }
}

fn select_log_file(sync_to_hot: bool) -> io::Result<PathBuf> {
    let data_dir = get_data_dir()?;

    if sync_to_hot {
        Ok(data_dir.join("backup_hot.log"))
    } else {
        Ok(data_dir.join("backup_cold.log"))
    }
}

pub fn run_backup_procedure(configs: &Configs) -> Result<(), BackupError> {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[*] -> Exit program");

    let option = read_option_from_stdin()?;

    if !(1..=4).contains(&option) {
        println!("Backup was manually aborted");
        return Ok(());
    }

    let sync_to_hot = matches!(option, 1 | 2);
    let is_dry_run = matches!(option, 2 | 4);

    let src = append_slash_to_src(&configs.source);
    let user = select_user(sync_to_hot, configs);
    let host = select_host(sync_to_hot, configs);
    let destination = select_destination(sync_to_hot, configs);

    if is_dry_run {
        subprocesses::run_rsync_dry_run(&src, user, host, destination)
    } else {
        let log_file = select_log_file(sync_to_hot)?;
        subprocesses::run_rsync(&src, user, host, destination, &log_file)
    }
}
