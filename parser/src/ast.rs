use internment::LocalIntern;
use std::{error::Error, fmt::Display, path::PathBuf, str::FromStr};

use crate::parser::Lexer;

use logos::internal::CallbackResult;
use logos::Logos;

#[derive(Debug)] 
pub enum Transformation {
    Forced {
        destruct: Expr,
        construct: Expr,
    }
} 
#[derive(Debug)] 
pub enum Expr {
    Number(f64),
    String(String, Option<StringFlag>),
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    Ident(LocalIntern<String>),
    Operator(Operator, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StringFlag {
    Format
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
        let mut lexer = Lexer::new(s, None);

        Ok(lexer.parse())
    }
}
