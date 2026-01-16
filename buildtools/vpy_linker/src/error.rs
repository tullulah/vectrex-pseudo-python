use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinkerError {
    #[error("Linker not yet implemented")]
    NotImplemented,

    #[error("Undefined symbol: {0}")]
    UndefinedSymbol(String),

    #[error("Multiple definitions: {0}")]
    MultipleDefinitions(String),

    #[error("Binary too large: {size} bytes exceeds {limit} bytes")]
    BinaryTooLarge { size: usize, limit: usize },
    
    #[error("Linker error: {0}")]
    Error(String),

    #[error("{0}")]
    Generic(String),
}

pub type LinkerResult<T> = Result<T, LinkerError>;
