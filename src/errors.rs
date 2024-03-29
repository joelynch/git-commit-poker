use std::io;

#[derive(Debug)]
pub enum LottoError {
    GitNotPresent(io::Error),
    GitFailed,
    ApplicationDirError(anyhow::Error),
}

impl From<io::Error> for LottoError {
    fn from(err: io::Error) -> LottoError {
        LottoError::GitNotPresent(err)
    }
}

impl std::fmt::Display for LottoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LottoError::GitNotPresent(_) => write!(f, "Could not find git on path"),
            LottoError::GitFailed => write!(f, "Git commit failed"),
            LottoError::ApplicationDirError(e) => {
                write!(f, "Error using application data directory: {}", e)
            }
        }
    }
}

impl std::error::Error for LottoError {}
