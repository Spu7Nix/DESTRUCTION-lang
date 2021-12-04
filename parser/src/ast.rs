use std::{str::FromStr, error::Error, fmt::Display, path::PathBuf};

type Pattern<'a> = Vec<Value<'a>>;

struct TopLevel<'a> {
    patts: Vec<Pattern<'a>>
}

enum Value<'a> {
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

impl<'a> Expr for Pattern<'a> {
    fn evaluate(&self) -> Value {
        todo!()
    }
}

impl<'a> Maths for Value<'a> {
    fn add(&self, other: &Self) -> Value {
        todo!()
    }

    fn sub(&self, other: &Self) -> Value {
        todo!()
    }

    fn div(&self, other: &Self) -> Value {
        todo!()
    }

    fn mul(&self, other: &Self) -> Value {
        todo!()
    }
}

impl FromStr for TopLevel<'_> {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

trait Maths {
    fn add(&self, other: &Self) -> Value;
    fn sub(&self, other: &Self) -> Value;
    fn div(&self, other: &Self) -> Value;
    fn mul(&self, other: &Self) -> Value;
}

trait Expr {
    fn evaluate(&self) -> Value;
}
