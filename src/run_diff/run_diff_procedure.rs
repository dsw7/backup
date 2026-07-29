use crate::configs::Configs;
use crate::errors::BackupError;

use super::subprocesses::{Usage, get_disk_usages};

fn get_usage_bytes(parts: &Vec<&str>) -> Result<usize, std::num::ParseIntError> {
    let bytes = match parts.first() {
        Some(val) => val.parse::<usize>()?,
        None => 0,
    };

    Ok(bytes)
}

fn bytes_to_human_readable(usage_bytes: usize) -> String {
    let units = ["B", "K", "M", "G", "T", "P", "E", "Z", "Y"];
    let mut size = usage_bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1}{}", size, units[unit_index])
}

fn get_path(parts: &Vec<&str>) -> String {
    match parts.get(1) {
        Some(val) => val.to_string(),
        None => String::from("-"),
    }
}

fn display_usages(usages: &Vec<Usage>) -> Result<(), BackupError> {
    println!(
        "{:<20} {:<25} {:<16} {}",
        "Host", "Path", "Usage (bytes)", "Usage"
    );
    println!(
        "{:<20} {:<25} {:<16} {}",
        "-------------------", "------------------------", "---------------", "-------"
    );

    for usage in usages {
        if let Usage::Success { host, stdout } = usage {
            let parts: Vec<&str> = stdout.split_whitespace().collect();
            let path = get_path(&parts);
            let usage_bytes = get_usage_bytes(&parts)?;
            let usage_bytes_human_readable = bytes_to_human_readable(usage_bytes);
            println!("{host:<20} {path:<25} {usage_bytes:<16} {usage_bytes_human_readable}");
        }
    }

    Ok(())
}

fn display_failed_usages(usages: &Vec<Usage>) {
    println!("{:<20} {:<25}", "Host", "Error");
    println!(
        "{:<20} {:<25}",
        "-------------------", "------------------------"
    );

    for usage in usages {
        if let Usage::Failure { host, stderr } = usage {
            println!("{host:<20} {stderr:<25}");
        }
    }
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(configs)?;

    display_usages(&usages)?;
    println!();
    display_failed_usages(&usages);

    Ok(())
}
