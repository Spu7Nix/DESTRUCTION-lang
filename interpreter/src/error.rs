use std::{error::Error, fmt::Display, path::PathBuf};

use parser::parser::Lexer;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    PatternMismatchT,
    PatternMismatch {
        pos: (usize, usize),
        message: String,
        file: Option<PathBuf>,
    },
    ValueErrorT,
    ValueError {
        pos: (usize, usize),
        message: String,
        file: Option<PathBuf>,
    }
}

impl RuntimeError {
    pub fn new(t: RuntimeError, lexer: &Lexer, message: &str) -> Self {
        match t {
            RuntimeError::PatternMismatchT => Self::PatternMismatch { file: lexer.file(), pos: lexer.pos(), message: message.to_string() },
            RuntimeError::ValueErrorT => Self::ValueError { file: lexer.file(), pos: lexer.pos(), message: message.to_string() },
            err @ RuntimeError::PatternMismatch { .. } => err,
            err @ RuntimeError::ValueError { .. } => err
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
