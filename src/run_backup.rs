use std::process::Command;

use crate::configs::Config;

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

fn run_rsync_hot_storage_dry_run(configs: &Config) {
    let src = append_slash_to_source(&configs.source);
    let store = &configs.storage.hot;
    let dst = format_destination(&store.user, &store.host, &store.destination);

    let status = Command::new("rsync")
        .arg("-av")
        .arg("--dry-run")
        .arg("--delete")
        .arg(src)
        .arg(dst)
        .status()
        .expect("Command failed to start. There is no way to proceed");

    println!("Process exited with status: {}", status);
}

pub fn run_backup(configs: &Config) {
    run_rsync_hot_storage_dry_run(&configs);
}
