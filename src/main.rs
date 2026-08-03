mod configs;
mod program_files;
mod run_backup;
mod run_diff;
mod run_init;

use clap::{Parser, Subcommand};

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
    /// Set up the program
    Init,

    /// Run backup procedure (this is the default)
    Backup,

    /// Compare local and remote backup directory sizes
    Diff,
}

fn run_command() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => initialize_program(),
        Some(Commands::Backup) => run_backup_procedure(),
        Some(Commands::Diff) => run_diff_procedure(),
        None => run_backup_procedure(),
    }
}

fn main() -> ExitCode {
    match run_command() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
