use crate::configs::Configs;

use std::io;
use std::process::{Command, Stdio};

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

pub fn get_disk_usages(configs: &Configs) -> io::Result<()> {
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

    let child_b = Command::new("ssh")
        .arg(get_ssh_dest_cold(&configs))
        .arg(format!("du --summarize --bytes {}", &configs.source))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output_a = child_a.wait_with_output()?;
    let output_b = child_b.wait_with_output()?;
    let output_c = child_c.wait_with_output()?;

    let stdout_a = String::from_utf8_lossy(&output_a.stdout);
    let stderr_a = String::from_utf8_lossy(&output_a.stderr);
    let stderr_c = String::from_utf8_lossy(&output_c.stderr);

    let stdout_b = String::from_utf8_lossy(&output_b.stdout);
    let stderr_b = String::from_utf8_lossy(&output_b.stderr);

    println!("{stdout_a}");
    println!("{stderr_a}");
    println!("{stdout_b}");
    println!("{stderr_b}");
    Ok(())
}
