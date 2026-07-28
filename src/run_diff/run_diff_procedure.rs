use crate::configs::Configs;
use crate::errors::BackupError;

use super::subprocesses::{Usage, get_disk_usages};

fn display_usages(usages: Vec<Usage>) {
    for usage in usages {
        println!("{} {} {}", usage.host, usage.path, usage.usage_bytes);
    }
}

fn display_failed_usages(failed_usages: Vec<Usage>) {
    for usage in failed_usages {
        println!("{} {}", usage.host, usage.stderr);
    }
}

fn partition_usages(usages: &mut Vec<Usage>) -> Vec<Usage> {
    usages.extract_if(.., |usage| !usage.failed).collect()
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let mut usages = get_disk_usages(&configs)?;
    let failed_usages = partition_usages(&mut usages);

    display_usages(usages);
    println!();
    display_failed_usages(failed_usages);

    Ok(())
}
