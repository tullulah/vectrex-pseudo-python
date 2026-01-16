use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{filename}:{line}:{col}: {message}")]
    SyntaxError {
        filename: String,
        line: usize,
        col: usize,
        message: String,
    },

    #[error("Unexpected EOF at line {line}")]
    UnexpectedEof { line: usize },

    #[error("Invalid token at line {line}: {token}")]
    InvalidToken { line: usize, token: String },

    #[error("{0}")]
    Generic(String),
}

impl ParseError {
    pub fn syntax_error(
        filename: impl Into<String>,
        line: usize,
        col: usize,
        message: impl Into<String>,
    ) -> Self {
        ParseError::SyntaxError {
            filename: filename.into(),
            line,
            col,
            message: message.into(),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
