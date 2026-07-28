use crate::configs::Configs;
use crate::errors::BackupError;

use super::subprocesses::{Usage, get_disk_usages};

fn display_usages(usages: &Vec<Usage>) {
    println!("{:<20} {:<25} {:<15}", "Host", "Path", "Usage (bytes)");
    println!(
        "{:<20} {:<25} {:<15}",
        "-------------------", "------------------------", "---------------"
    );

    for usage in usages {
        if let Usage::Success {
            host,
            path,
            usage_bytes,
        } = usage
        {
            println!("{:<20} {:<25} {:<15}", host, path, usage_bytes);
        }
    }
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

    display_usages(&usages);
    println!();
    display_failed_usages(&usages);

    Ok(())
}
