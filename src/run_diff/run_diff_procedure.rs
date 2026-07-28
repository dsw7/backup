use crate::configs::Configs;
use crate::errors::BackupError;

use std::io;
use std::process::Command;

fn get_disk_usages(configs: &Configs) -> io::Result<()> {
    let mut proc_local = Command::new("du")
        .args(["--summarize", "--bytes", &configs.source])
        .spawn()?;

    let status = proc_local.wait()?;
    println!("{status}");
    Ok(())
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(&configs)?;

    Ok(())
}
