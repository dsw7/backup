mod configs;
mod data_directory;
mod errors;
mod run_backup;

use clap::{Parser, Subcommand};

use configs::load_configs;
use errors::BackupError;
use run_backup::run_backup_procedure;

use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "backup", version, about = "DSW backup and recovery system")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run backup procedure (this is the default)
    Backup,

    /// Report when backups were made
    Latest,
}

fn run_command() -> Result<(), BackupError> {
    let cli = Cli::parse();
    let configs = load_configs()?;

    match &cli.command {
        Some(Commands::Latest) => unimplemented!("Not ready!"),
        Some(Commands::Backup) => run_backup_procedure(&configs),
        None => run_backup_procedure(&configs),
    }
}

fn main() -> ExitCode {
    match run_command() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
