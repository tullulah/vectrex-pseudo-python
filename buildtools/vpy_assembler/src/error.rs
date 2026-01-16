use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum AssemblerError {
    #[error("Assembler not yet implemented")]
    NotImplemented,

    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Unknown label: {0}")]
    UnknownLabel(String),
    
    #[error("Assembly error at line {line}: {msg}")]
    Error { line: usize, msg: String },
    
    #[error("Assembly failed: {0}")]
    Failed(String),

    #[error("{0}")]
    Generic(String),
}

// Alias for compatibility with lib.rs
pub type AssemblyError = AssemblerError;
