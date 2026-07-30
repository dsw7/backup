mod configs;
mod data_directory;
mod errors;
mod run_backup;
mod run_diff;

use clap::{Parser, Subcommand};

use configs::load_configs;
use errors::BackupError;
use run_backup::run_backup_procedure;
use run_diff::run_diff_procedure;

use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "backup",
    version,
    about = "DSW backup and recovery system",
    after_help = "See https://github.com/dsw7/backup for more information
See the rsync(1) manpages for a more general backup and recovery command"
)]
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

    /// Compare local and remote backup directory sizes
    Diff,
}

fn run_command() -> Result<(), BackupError> {
    let cli = Cli::parse();
    let configs = load_configs()?;

    match &cli.command {
        Some(Commands::Backup) => run_backup_procedure(&configs),
        Some(Commands::Diff) => run_diff_procedure(&configs),
        Some(Commands::Latest) => unimplemented!("Not ready!"),
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
