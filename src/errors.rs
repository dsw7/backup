use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("An I/O error occurred: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("A parsing error occurred: {source}")]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("Failed to load configurations: {source}")]
    Configs {
        #[from]
        source: toml::de::Error,
    },
}
