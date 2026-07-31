use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn get_home_dir() -> io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Couldn't get home directory",
        )),
    }
}

fn write_readme_contents(path_readme: &Path) -> io::Result<()> {
    let contents = "* DSW data backup system
* For more information, see https://github.com/dsw7/backup";

    let mut readme = File::create(path_readme)?;
    readme.write_all(contents.as_bytes())?;
    Ok(())
}

fn manage_readme(program_dotdir: &Path) -> io::Result<()> {
    let path_readme = program_dotdir.join("README.txt");

    if !path_readme.exists() {
        write_readme_contents(&path_readme)?;
    }

    Ok(())
}

pub fn get_app_dir() -> io::Result<PathBuf> {
    let home_dir = get_home_dir()?;
    let program_dotdir = home_dir.join(".backup");

    if program_dotdir.exists() {
        manage_readme(&program_dotdir)?;
        Ok(program_dotdir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Program data directory does not exist",
        ))
    }
}
