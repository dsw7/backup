mod configs;
mod errors;
mod program_files;
mod run_backup;

use configs::load_configs;
use errors::BackupError;
use run_backup::run_backup;

use std::process::ExitCode;

fn run_sync() -> Result<String, BackupError> {
    let configs = load_configs()?;
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
