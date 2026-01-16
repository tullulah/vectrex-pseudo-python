use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnifierError {
    #[error("Unifier not yet implemented")]
    NotImplemented,

    #[error("Unresolved symbol: {0}")]
    UnresolvedSymbol(String),

    #[error("Import not found: {0}")]
    ImportNotFound(String),

    #[error("Circular dependency: {0}")]
    CircularDependency(String),

    #[error("Symbol conflict: {0}")]
    SymbolConflict(String),

    #[error("{0}")]
    Generic(String),
}

pub type UnifierResult<T> = Result<T, UnifierError>;
