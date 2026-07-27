use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("Failed to load configurations: {0}")]
    ConfigurationError(String),

    #[error("{0}")]
    Other(String),
}
