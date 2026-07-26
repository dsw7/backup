mod configs;
mod data_directory;
mod errors;
mod run_backup;

use clap::Parser;

use configs::load_configs;
use errors::BackupError;
use run_backup::run_backup_procedure;

use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "backup", version, about = "DSW backup and recovery system")]
struct Cli {}

fn run_sync() -> Result<String, BackupError> {
    let configs = load_configs()?;
    run_backup_procedure(&configs)
}

fn main() -> ExitCode {
    Cli::parse();

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
