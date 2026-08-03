use crate::program_files;
use anyhow::Context;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn log_new_file(path: &Path) {
    println!("(+) {}", path.display());
}

fn log_already_exists(path: &Path) {
    println!("Already exists: {}", path.display());
}

fn create_app_dir() -> anyhow::Result<PathBuf> {
    let appdir = program_files::get_app_dir()?;

    if appdir.exists() {
        log_already_exists(&appdir);
    } else {
        fs::create_dir(&appdir).context(format!("Failed to create {}", appdir.display()))?;
        log_new_file(&appdir);
    }

    Ok(appdir)
}

fn write_readme_contents(path_readme: &PathBuf) -> anyhow::Result<()> {
    let contents = "* DSW data backup system
* For more information, see https://github.com/dsw7/backup";

    let mut readme = fs::File::create(path_readme)
        .context(format!("Could not create {}", path_readme.display()))?;

    readme
        .write_all(contents.as_bytes())
        .context(format!("Could not write to {}", path_readme.display()))?;

    Ok(())
}

fn write_readme(app_dir: &Path) -> anyhow::Result<()> {
    let path_readme = program_files::get_readme_file(app_dir);

    if path_readme.exists() {
        log_already_exists(&path_readme);
    } else {
        write_readme_contents(&path_readme)?;
        log_new_file(&path_readme);
    }

    Ok(())
}

fn write_config_file_contents(config_file: &PathBuf) -> anyhow::Result<()> {
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

    let mut configs = fs::File::create(config_file).context(format!(
        "Could not create the configuration file: {}",
        config_file.display()
    ))?;

    configs
        .write_all(contents.as_bytes())
        .context(format!("Could not write to {}", config_file.display()))?;

    Ok(())
}

fn write_config_file(app_dir: &Path) -> anyhow::Result<()> {
    let config_file = program_files::get_config_file(app_dir);

    if config_file.exists() {
        log_already_exists(&config_file);
    } else {
        write_config_file_contents(&config_file)?;
        log_new_file(&config_file);
    }

    Ok(())
}

pub fn initialize_program() -> anyhow::Result<()> {
    let app_dir = create_app_dir()?;
    write_readme(&app_dir)?;
    write_config_file(&app_dir)?;
    Ok(())
}
