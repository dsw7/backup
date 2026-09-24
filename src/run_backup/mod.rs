use anyhow::Context;

use crate::configs::load_configs;

mod rsync_dry_run;
mod rsync_live_run;
mod run_backup_procedure;

use run_backup_procedure::run_data_backup;

pub fn run_backup_procedure() -> anyhow::Result<()> {
    let configs = load_configs().context("failed to load configurations")?;
    run_data_backup(&configs).context("backup procedure failed")?;

    Ok(())
}
