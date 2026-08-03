use anyhow::Context;

use crate::configs::load_configs;

mod run_diff_procedure;
mod subprocesses;

pub fn run_diff_procedure() -> anyhow::Result<()> {
    let configs = load_configs().context("Failed to load configurations")?;
    self::run_diff_procedure::get_diff_between_machines(&configs)
}
