use crate::configs::load_configs;
use crate::errors::BackupError;

mod rsync_dry_run;
mod rsync_live_run;
mod run_backup_procedure;

pub fn run_backup_procedure() -> Result<(), BackupError> {
    let configs = load_configs()?;
    self::run_backup_procedure::run_data_backup(&configs)
}
