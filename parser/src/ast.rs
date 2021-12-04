use std::{str::FromStr, error::Error, fmt::Display, path::PathBuf};

pub type Pattern<'a> = Vec<Value<'a>>;

struct TopLevel<'a> {
    patts: Vec<Pattern<'a>>
}

pub enum Value<'a> {
    Number(f64),
    String(String),
    Array(&'a [Value<'a>]),
    Tuple(&'a [Value<'a>])
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

impl FromStr for TopLevel<'_> {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
