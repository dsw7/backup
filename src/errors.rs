use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("An integer parsing error occurred: {source}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("Failed to load TOML configurations: {source}")]
    TOMLError {
        #[from]
        source: toml::de::Error,
    },
}
