use std::fmt;

#[derive(Debug, Clone)]
pub struct BackupError(pub String);

impl fmt::Display for BackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for BackupError {
    fn from(msg: String) -> Self {
        BackupError(msg)
    }
}
