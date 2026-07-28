use crate::configs::Configs;
use crate::errors::BackupError;

use std::io;
use std::process::{Command, Stdio};

fn get_ssh_dest_hot(configs: &Configs) -> String {
    format!(
        "{}@{}",
        &configs.storage.hot.user, &configs.storage.hot.host
    )
}

fn get_disk_usages(configs: &Configs) -> io::Result<()> {
    let child_a = Command::new("du")
        .args(["--summarize", "--bytes", &configs.source])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let child_b = Command::new("ssh")
        .arg(get_ssh_dest_hot(&configs))
        .arg(format!("du --summarize --bytes {}", &configs.source))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status_a = child_a.wait_with_output()?;
    let status_b = child_b.wait_with_output()?;

    let stdout_a = String::from_utf8_lossy(&status_a.stdout);
    let stderr_a = String::from_utf8_lossy(&status_a.stderr);

    let stdout_b = String::from_utf8_lossy(&status_b.stdout);
    let stderr_b = String::from_utf8_lossy(&status_b.stderr);

    println!("{stdout_a}");
    println!("{stderr_a}");
    println!("{stdout_b}");
    println!("{stderr_b}");
    Ok(())
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(&configs)?;

    Ok(())
}
