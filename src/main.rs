mod configs;
mod errors;
mod program_files;
mod run_backup;
mod user_selection;

use configs::load_configs;
use errors::BackupError;
use run_backup::run_backup;
use user_selection::select_backup_type;

use std::process::ExitCode;

fn run_sync() -> Result<String, BackupError> {
    let configs = load_configs()?;
    select_backup_type();
    run_backup(&configs)
}

fn main() -> ExitCode {
    match run_sync() {
        Ok(results) => {
            println!("{results}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
