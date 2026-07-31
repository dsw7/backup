use crate::errors::BackupError;
use crate::program_files;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn log_new_file(path: &Path) {
    println!("(+) {}", path.display());
}

fn create_app_dir() -> io::Result<PathBuf> {
    let appdir = program_files::get_app_dir()?;

    if appdir.exists() {
        println!("Program application directory already exists. No action taken");
    } else {
        fs::create_dir(&appdir)?;
        log_new_file(&appdir);
    }

    Ok(appdir)
}

fn write_readme_contents(path_readme: &PathBuf) -> io::Result<()> {
    let contents = "* DSW data backup system
* For more information, see https://github.com/dsw7/backup";

    let mut readme = fs::File::create(path_readme)?;
    readme.write_all(contents.as_bytes())?;
    Ok(())
}

fn write_readme(app_dir: &Path) -> io::Result<()> {
    let path_readme = program_files::get_readme_file(app_dir);

    if path_readme.exists() {
        println!("Program README already exists. No action taken");
    } else {
        write_readme_contents(&path_readme)?;
        log_new_file(&path_readme);
    }

    Ok(())
}

fn write_config_file_contents(config_file: &PathBuf) -> io::Result<()> {
    let contents = r#"# Specify where to sync data from
source = ""

[storage.hot]
user = ""
host = ""
destination = ""

[storage.cold]
user = ""
host = ""
destination = ""
"#;

    let mut configs = fs::File::create(config_file)?;
    configs.write_all(contents.as_bytes())?;
    Ok(())
}

fn write_config_file(app_dir: &Path) -> io::Result<()> {
    let config_file = program_files::get_config_file(app_dir);

    if config_file.exists() {
        println!("Program config file already exists. No action taken");
    } else {
        write_config_file_contents(&config_file)?;
        log_new_file(&config_file);
    }

    Ok(())
}

pub fn initialize_program() -> Result<(), BackupError> {
    let app_dir = create_app_dir()?;
    write_readme(&app_dir)?;
    write_config_file(&app_dir)?;
    Ok(())
}
