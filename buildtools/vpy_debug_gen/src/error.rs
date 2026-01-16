use thiserror::Error;

#[derive(Debug, Error)]
pub enum DebugError {
    #[error("Debug generation not yet implemented")]
    NotImplemented,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("{0}")]
    Generic(String),
}

pub type DebugResult<T> = Result<T, DebugError>;
