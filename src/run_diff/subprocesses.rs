use crate::configs::Configs;
use crate::errors::BackupError;

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

fn get_usage_and_path_from_stdout(stdout: &String) -> Result<(usize, String), BackupError> {
    let parts: Vec<&str> = stdout.split_whitespace().collect();

    if parts.len() != 2 {
        return Err(BackupError::Other(
            "The stdout does not contain exactly two components".into(),
        ));
    }

    let usage_bytes = parts[0].parse::<usize>()?;
    let path = parts[1].to_owned();
    Ok((usage_bytes, path))
}

pub enum Usage {
    Success {
        host: String,
        path: String,
        usage_bytes: usize,
    },
    Failure {
        host: String,
        stderr: String,
    },
}

fn unpack_output(host: &String, output: &Output) -> Result<Usage, BackupError> {
    let result = if output.status.success() {
        let stdout = extract_stdout(&output);
        let (usage_bytes, path) = get_usage_and_path_from_stdout(&stdout)?;

        Usage::Success {
            host: String::from(host),
            path: path,
            usage_bytes: usage_bytes,
        }
    } else {
        Usage::Failure {
            host: String::from(host),
            stderr: extract_stderr(&output),
        }
    };

    Ok(result)
}

pub fn get_disk_usages(configs: &Configs) -> Result<Vec<Usage>, BackupError> {
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

    let usage_localhost = unpack_output(&String::from("localhost"), &output_localhost)?;
    let usage_hot_backup = unpack_output(&configs.storage.hot.host, &output_hot_backup)?;
    let usage_cold_backup = unpack_output(&configs.storage.cold.host, &output_cold_backup)?;

    Ok(vec![usage_localhost, usage_hot_backup, usage_cold_backup])
}
