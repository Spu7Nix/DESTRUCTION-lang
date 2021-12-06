use std::{path::PathBuf, fmt::Display, error::Error};

#[derive(Debug)]
pub enum LangError {
    SyntaxError {
        pos: (usize, usize),
        message: String,
        file: Option<PathBuf>,
    },
}

pub enum LangErrorT {
    SyntaxError
}

impl Error for LangError {}

impl Display for LangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}