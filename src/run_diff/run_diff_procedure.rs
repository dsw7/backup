use crate::configs::Configs;
use crate::errors::BackupError;

use std::io;
use std::process::Command;

fn get_disk_usages(configs: &Configs) -> io::Result<()> {
    let mut child_a = Command::new("du")
        .args(["--summarize", "--bytes", &configs.source])
        .spawn()?;

    let mut child_b = Command::new("ssh")
        .arg(format!(
            "{}@{}",
            &configs.storage.hot.user, &configs.storage.hot.host
        ))
        .arg(format!("du --summarize --bytes {}", &configs.source))
        .spawn()?;

    let status_a = child_a.wait()?;
    let status_b = child_b.wait()?;
    println!("{status_a}");
    println!("{status_b}");
    Ok(())
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(&configs)?;

    Ok(())
}
