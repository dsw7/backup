use crate::configs::Configs;
use crate::errors::BackupError;

use super::subprocesses::{Usage, get_disk_usages};

fn get_usage_bytes(parts: &Vec<&str>) -> Result<usize, std::num::ParseIntError> {
    let bytes = match parts.get(0) {
        Some(val) => val.parse::<usize>()?,
        None => 0,
    };

    Ok(bytes)
}

fn get_path(parts: &Vec<&str>) -> String {
    match parts.get(1) {
        Some(val) => val.to_string(),
        None => String::from("-"),
    }
}

fn display_usages(usages: &Vec<Usage>) -> Result<(), BackupError> {
    println!("{:<20} {:<25} {:<15}", "Host", "Path", "Usage (bytes)");
    println!(
        "{:<20} {:<25} {:<15}",
        "-------------------", "------------------------", "---------------"
    );

    for usage in usages {
        if let Usage::Success { host, stdout } = usage {
            let parts: Vec<&str> = stdout.split_whitespace().collect();
            let path = get_path(&parts);
            let usage_bytes = get_usage_bytes(&parts)?;
            println!("{:<20} {:<25} {:<15}", host, path, usage_bytes);
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
            println!("{:<20} {:<25}", host, stderr);
        }
    }
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(&configs)?;

    display_usages(&usages)?;
    println!();
    display_failed_usages(&usages);

    Ok(())
}
