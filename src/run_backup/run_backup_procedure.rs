use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;

use crate::configs::Configs;

use super::rsync_dry_run;
use super::rsync_live_run;

fn read_option_from_stdin() -> io::Result<i32> {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(i32::from_str(input.trim()).unwrap_or_default())
}

fn append_slash_to_src(src: &String) -> String {
    if src.ends_with('/') {
        String::from(src)
    } else {
        format!("{src}/")
    }
}

fn remove_slash_from_dst(dst: &String) -> String {
    if dst.ends_with('/') {
        String::from(&dst[..dst.len() - 1])
    } else {
        String::from(dst)
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

fn select_log_file(sync_to_hot: bool) -> PathBuf {
    if sync_to_hot {
        PathBuf::from("backup_hot.log")
    } else {
        PathBuf::from("backup_cold.log")
    }
}

pub fn run_data_backup(configs: &Configs) -> anyhow::Result<()> {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[*] -> Exit program");

    let option = read_option_from_stdin()
        .context("Something went wrong when communicating with stdin/stdout")?;

    if !(1..=4).contains(&option) {
        println!("Backup was manually aborted");
        return Ok(());
    }

    let sync_to_hot = matches!(option, 1 | 2);
    let is_dry_run = matches!(option, 2 | 4);

    let src = append_slash_to_src(&configs.source);
    let user = select_user(sync_to_hot, configs);
    let host = select_host(sync_to_hot, configs);
    let dst = select_destination(sync_to_hot, configs);
    let dst = remove_slash_from_dst(dst);

    if is_dry_run {
        rsync_dry_run::run_rsync_subprocess(&src, user, host, &dst)
    } else {
        let log_file = select_log_file(sync_to_hot);
        rsync_live_run::run_rsync_subprocess(&src, user, host, &dst, &log_file)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_append_slash_to_src() {
        assert_eq!(super::append_slash_to_src(&"/tmp/bar".into()), "/tmp/bar/");
        assert_eq!(super::append_slash_to_src(&"/tmp/bar/".into()), "/tmp/bar/");
    }

    #[test]
    fn test_remove_slash_from_dst() {
        assert_eq!(super::remove_slash_from_dst(&"/tmp/bar".into()), "/tmp/bar");
        assert_eq!(
            super::remove_slash_from_dst(&"/tmp/bar/".into()),
            "/tmp/bar"
        );
    }
}
