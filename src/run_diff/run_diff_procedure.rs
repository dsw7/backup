use crate::configs::Configs;
use crate::errors::BackupError;

use super::subprocesses::get_disk_usages;

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(&configs)?;

    Ok(())
}
