use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("Failed to load configurations: {0}")]
    ConfigurationError(String),

    #[error("Subprocess exited with code {0}")]
    SubprocessError(i32),

    #[error("{0}")]
    Other(String),
}
