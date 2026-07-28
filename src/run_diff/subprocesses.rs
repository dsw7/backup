use crate::configs::Configs;

use std::io;
use std::process::{Command, Output, Stdio};

fn get_ssh_dest_hot(configs: &Configs) -> String {
    format!(
        "{}@{}",
        &configs.storage.hot.user, &configs.storage.hot.host
    )
}

fn get_ssh_dest_cold(configs: &Configs) -> String {
    format!(
        "{}@{}",
        &configs.storage.cold.user, &configs.storage.cold.host
    )
}

pub struct Usage {
    pub failed: bool,
    pub host: String,
    pub stderr: String,
    pub stdout: String,
}

fn unpack_output(host: &String, output: &Output) -> Usage {
    let stdout = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        String::from("-")
    };

    Usage {
        failed: output.status.success(),
        host: String::from(host),
        stdout: stdout,
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

pub fn get_disk_usages(configs: &Configs) -> io::Result<Vec<Usage>> {
    let proc_localhost = Command::new("du")
        .args(["--summarize", "--bytes", &configs.source])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let proc_hot_backup = Command::new("ssh")
        .arg(get_ssh_dest_hot(&configs))
        .arg(format!("du --summarize --bytes {}", &configs.source))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let proc_cold_backup = Command::new("ssh")
        .arg(get_ssh_dest_cold(&configs))
        .arg(format!("du --summarize --bytes {}", &configs.source))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output_localhost = proc_localhost.wait_with_output()?;
    let output_hot_backup = proc_hot_backup.wait_with_output()?;
    let output_cold_backup = proc_cold_backup.wait_with_output()?;

    let usages = vec![
        unpack_output(&String::from("localhost"), &output_localhost),
        unpack_output(&configs.storage.hot.host, &output_hot_backup),
        unpack_output(&configs.storage.cold.host, &output_cold_backup),
    ];

    Ok(usages)
}
