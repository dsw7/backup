mod configs;
mod errors;
mod program_files;
mod run_backup;

use configs::load_configs;
use run_backup::run_backup;

use std::process::ExitCode;

fn main() -> ExitCode {
    let configs = match load_configs() {
        Ok(configs) => configs,
        Err(error) => {
            eprintln!("Failed to load program configurations: {error}");
            return ExitCode::FAILURE;
        }
    };

    match run_backup(&configs) {
        Ok(results) => {
            println!("{results}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Data synchronization failed: {error}");
            ExitCode::FAILURE
        }
    }
}
