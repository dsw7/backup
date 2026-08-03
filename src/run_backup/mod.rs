use crate::configs::load_configs;

mod rsync_dry_run;
mod rsync_live_run;
mod run_backup_procedure;

pub fn run_backup_procedure() -> anyhow::Result<()> {
    let configs = load_configs()?;
    self::run_backup_procedure::run_data_backup(&configs)
}
