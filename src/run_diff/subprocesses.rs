use crate::configs::Configs;

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

fn extract_stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn extract_stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_owned()
}

pub enum Usage {
    Success { host: String, stdout: String },
    Failure { host: String, stderr: String },
}

fn unpack_output(host: &String, output: &Output) -> Usage {
    if output.status.success() {
        Usage::Success {
            host: String::from(host),
            stdout: extract_stdout(output),
        }
    } else {
        Usage::Failure {
            host: String::from(host),
            stderr: extract_stderr(output),
        }
    }
}

pub fn get_disk_usages(configs: &Configs) -> anyhow::Result<Vec<Usage>, > {
    let proc_localhost = Command::new("du")
        .args(["--summarize", "--bytes", &configs.source])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let proc_hot_backup = Command::new("ssh")
        .arg(get_ssh_dest_hot(configs))
        .arg(format!(
            "du --summarize --bytes {}",
            &configs.storage.hot.destination
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let proc_cold_backup = Command::new("ssh")
        .arg(get_ssh_dest_cold(configs))
        .arg(format!(
            "du --summarize --bytes {}",
            &configs.storage.cold.destination
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output_localhost = proc_localhost.wait_with_output()?;
    let output_hot_backup = proc_hot_backup.wait_with_output()?;
    let output_cold_backup = proc_cold_backup.wait_with_output()?;

    let results = vec![
        unpack_output(&String::from("localhost"), &output_localhost),
        unpack_output(&configs.storage.hot.host, &output_hot_backup),
        unpack_output(&configs.storage.cold.host, &output_cold_backup),
    ];

    Ok(results)
}
