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

fn extract_stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn extract_stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_owned()
}

fn get_stdout_parts(stdout: &String) -> (String, String) {
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    let usage_bytes = parts[0].to_owned();
    let path = parts[1].to_owned();

    (usage_bytes, path)
}

pub struct Usage {
    pub failed: bool,
    pub host: String,
    pub stderr: String,
    pub path: String,
    pub usage_bytes: String,
}

fn unpack_output(host: &String, output: &Output) -> Usage {
    let (usage_bytes, path) = if output.status.success() {
        let stdout = extract_stdout(&output);
        get_stdout_parts(&stdout)
    } else {
        (String::from("-"), String::from("-"))
    };

    Usage {
        failed: output.status.success(),
        host: String::from(host),
        stderr: extract_stderr(&output),
        path: path,
        usage_bytes: usage_bytes,
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
