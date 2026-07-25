use std::fmt;

#[derive(Debug, Clone)]
pub struct ConfigError(pub String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ConfigError {
    fn from(msg: String) -> Self {
        ConfigError(msg)
    }
}
