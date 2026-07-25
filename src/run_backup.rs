use std::process::Command;

use crate::configs::Config;

fn append_slash_to_source(source: &String) -> String {
    if source.ends_with('/') {
        String::from(source)
    } else {
        String::from(format!("{source}/"))
    }
}

fn run_rsync_hot_storage_dry_run(configs: &Config) {
    let src = append_slash_to_source(&configs.source);
    let dst = String::from("dsw@10.0.0.115:/tmp/bar");
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
