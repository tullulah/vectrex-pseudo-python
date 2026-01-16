use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    Generic(String),
}

pub type WriterResult<T> = Result<T, WriterError>;
