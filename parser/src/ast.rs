use std::{str::FromStr, error::Error, fmt::Display, path::PathBuf};

pub type Pattern = Vec<Value>;
pub type Transformation = Vec<Pattern>;

struct TopLevel {
    exprs: Vec<Transformation>
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>)
}

#[derive(Debug)]
pub enum LangError {
    SyntaxError { pos: (usize, usize), message: String, file: Option<PathBuf> }
}

impl Error for LangError {}

impl Display for LangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Pretty print for errors")
    }
}

impl FromStr for TopLevel {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
