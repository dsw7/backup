use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("{0}")]
    Other(String),
}
