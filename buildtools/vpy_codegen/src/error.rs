use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Codegen not yet implemented")]
    NotImplemented,

    #[error("Unknown builtin: {0}")]
    UnknownBuiltin(String),

    #[error("Invalid argument count for {fn_name}: expected {expected}, got {actual}")]
    InvalidArity {
        fn_name: String,
        expected: usize,
        actual: usize,
    },

    #[error("{0}")]
    Generic(String),
}

pub type CodegenResult<T> = Result<T, CodegenError>;
