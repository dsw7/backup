use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },

    #[error("An integer parsing error occurred: {source}")]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("Failed to load TOML configurations: {source}")]
    ParseTOML {
        #[from]
        source: toml::de::Error,
    },
}
