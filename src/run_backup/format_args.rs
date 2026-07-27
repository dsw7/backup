use crate::configs::Configs;
use crate::data_directory;

use std::io;
use std::path::PathBuf;

pub fn format_src(source: &String) -> String {
    if source.ends_with('/') {
        String::from(source)
    } else {
        String::from(format!("{source}/"))
    }
}

fn format_dst(user: &String, host: &String, destination: &String) -> String {
    let dst = if destination.ends_with('/') {
        String::from(&destination[..destination.len() - 1])
    } else {
        String::from(destination)
    };

    format!("{user}@{host}:{dst}")
}

pub fn format_dst_hot(configs: &Configs) -> String {
    format_dst(
        &configs.storage.hot.user,
        &configs.storage.hot.host,
        &configs.storage.hot.destination,
    )
}

pub fn format_dst_cold(configs: &Configs) -> String {
    format_dst(
        &configs.storage.cold.user,
        &configs.storage.cold.host,
        &configs.storage.cold.destination,
    )
}

pub fn select_log_file(sync_to_hot: bool) -> io::Result<PathBuf> {
    let program_dir = data_directory::get_data_dir()?;

    if sync_to_hot {
        Ok(PathBuf::from(program_dir).join("backup_hot.log"))
    } else {
        Ok(PathBuf::from(program_dir).join("backup_cold.log"))
    }
}
