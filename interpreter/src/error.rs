use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RuntimeError {
    PatternMismatch,
    ValueError
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
