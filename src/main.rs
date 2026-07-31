mod configs;
mod data_directory;
mod errors;
mod run_backup;
mod run_diff;
mod run_init;

use clap::{Parser, Subcommand};

use errors::BackupError;
use run_backup::run_backup_procedure;
use run_diff::run_diff_procedure;
use run_init::initialize_program;

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

    /// Set up the program
    Init,

    /// Compare local and remote backup directory sizes
    Diff,

    /// Report when backups were made
    Latest,
}

fn run_command() -> Result<(), BackupError> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => initialize_program(),
        Some(Commands::Backup) => run_backup_procedure(),
        Some(Commands::Diff) => run_diff_procedure(),
        Some(Commands::Latest) => unimplemented!("Not ready!"),
        None => run_backup_procedure(),
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
