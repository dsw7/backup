use thiserror::Error;

use std::io;
use std::num;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("A parsing error occurred: {source}")]
    ParseInt {
        #[from]
        source: num::ParseIntError,
    },

    #[error("Failed to load configurations: {0}")]
    ConfigurationError(String),
}
