use internment::LocalIntern;
use std::str::FromStr;

use crate::{error::LangError, parser::{Lexer, Sp}};

type Expression = Sp<Expr>;

#[derive(Debug)]
pub enum Transformation {
    Forced { destruct: Expr, construct: Expr },
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
    Format,
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

impl FromStr for TopLevel {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer::new(s, None);

        Ok(lexer.parse())
    }
}
