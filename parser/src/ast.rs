use internment::LocalIntern;
use std::{error::Error, fmt::Display, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub enum Transformation {
    Number(f64),
    String(String),
    Array(Vec<Transformation>),
    Tuple(Vec<Transformation>),
    Ident(LocalIntern<String>),
    Operator(Box<Transformation>, Operator, Box<Transformation>),
    Change(Box<Transformation>, Box<Transformation>), // arrow ->
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct TopLevel {
    pub transformations: Vec<Transformation>,
}

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

impl FromStr for TopLevel {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
