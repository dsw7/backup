use crate::errors::BackupError;
use crate::program_files;

use std::fs;
use std::io;
use std::path::PathBuf;

fn create_app_dir() -> io::Result<PathBuf> {
    let appdir = program_files::get_app_dir()?;

    if appdir.exists() {
        println!(
            "Program application directory `{}` already exists",
            appdir.display()
        );
    } else {
        fs::create_dir(&appdir)?;
        println!(
            "Created program application directory: `{}`",
            appdir.display()
        );
    }

    Ok(appdir)
}

pub fn initialize_program() -> Result<(), BackupError> {
    let _ = create_app_dir()?;
    Ok(())
}
