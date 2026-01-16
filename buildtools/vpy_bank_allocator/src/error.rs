use thiserror::Error;

#[derive(Debug, Error)]
pub enum BankAllocatorError {
    #[error("Code too large for single bank: {0} bytes")]
    CodeTooLarge(usize),

    #[error("Function too large: {0} ({1} bytes)")]
    FunctionTooLarge(String, usize),

    #[error("Circular dependency in call graph")]
    CircularDependency,

    #[error("{0}")]
    Generic(String),
}

pub type BankAllocatorResult<T> = Result<T, BankAllocatorError>;
