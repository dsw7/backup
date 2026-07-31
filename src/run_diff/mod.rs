use crate::configs::load_configs;
use crate::errors::BackupError;

mod run_diff_procedure;
mod subprocesses;

pub fn run_diff_procedure() -> Result<(), BackupError> {
    let configs = load_configs()?;
    self::run_diff_procedure::get_diff_between_machines(&configs)
}
