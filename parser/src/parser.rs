use logos::Logos;
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use crate::ast::{LangError, LangErrorT, TopLevel, Transformation};

type Token = Sp<Tokens>;

#[derive(Clone)]
pub struct Lexer<'a> {
    pos: (usize, usize),
    tokens: logos::Lexer<'a, Tokens>,
    file: Option<PathBuf>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str, file: Option<PathBuf>) -> Self {
        let lexer = logos::Lexer::new(content);
        Self {
            tokens: lexer,
            pos: (0, 0),
            file,
        }
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.next()?;

        if token == Tokens::Newline {
            self.pos.0 += 1;
        } else {
            self.pos.1 += self.tokens.span().len();
        }

        Some(Token {
            data: token,
            span: self.tokens.span().into(),
        })
    }

    pub fn parse(&mut self) -> TopLevel {
        let mut top_level = TopLevel {
            transformations: Vec::new(),
        };

        loop {
            if self.peek().is_none() {
                break;
            }
            top_level.transformations.push(self.parse_transform());
        }

        top_level
    }

    fn parse_transform(&mut self) -> Transformation {
        let next_token = self.next_token().unwrap_or_else(|| {
            self.throw_error(LangErrorT::SyntaxError, "Unexpected end of input")
        });

        let first = match next_token.data {
            Tokens::Number(n) => Transformation::Number(n),
            Tokens::StringLiteral(s) => Transformation::String(s),

            Tokens::Lbracket => {
                let mut transforms = Vec::new();
                loop {
                    transforms.push(self.parse_transform());
                    match self
                        .next_token()
                        .unwrap_or_else(|| {
                            self.throw_error(LangErrorT::SyntaxError, "Unexpected end of input");
                        })
                        .data
                    {
                        Tokens::Comma => (),
                        Tokens::Rbracket => break,
                        token => self.throw_error(
                            LangErrorT::SyntaxError,
                            &format!("Expected tokens `Rbracket` or `Comma`, found {:?}", token),
                        ),
                    }
                }
                Transformation::Array(transforms)
            }

            Tokens::Ident(s) => Transformation::Ident(s),

            token => self.throw_error(
                LangErrorT::SyntaxError,
                &format!("Unexpected token {:?}", token),
            ),
        };

        if let Some(Token {
            data: Tokens::Rarrow,
            ..
        }) = self.peek()
        {
            self.next_token().unwrap_or_else(|| {
                self.throw_error(LangErrorT::SyntaxError, "Unexpected end of input");
            });
            let second = self.parse_transform();
            Transformation::Change(first.into(), second.into())
        } else {
            first
        }
    }

    pub fn throw_error(&self, error: LangErrorT, message: &str) -> ! {
        let error = match error {
            LangErrorT::SyntaxError => LangError::SyntaxError {
                file: self.file.to_owned(),
                pos: self.pos,
                message: message.to_owned(),
            },
        };
        println!("{}", error);

        std::process::exit(1)
    }

    pub fn peek(&self) -> Option<Token> {
        // Cloning self.tokens is more efficient than cloning self, 1 field vs 3
        let mut tokens = self.tokens.clone();
        let token = tokens.next()?;

        Some(Token {
            data: token,
            span: tokens.span().into(),
        })
    }

    pub fn peek_many(&self, amount: usize) -> Vec<Token> {
        let mut out = Vec::with_capacity(amount);
        let mut tokens = self.tokens.clone();
        let mut idx = 0;

        while let Some(token) = tokens.next() {
            if idx == amount {
                break;
            }
            out.push(Token {
                data: token,
                span: tokens.span().into(),
            });
            idx += 1;
        }

        out
    }

    pub(crate) fn pos(&self) -> (usize, usize) {
        self.pos
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sp<T> {
    data: T,
    span: Span,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            data: Tokens::Error,
            span: Span::new(0, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<logos::Span> for Span {
    fn from(range: logos::Span) -> Self {
        Span::new(range.start, range.end)
    }
}

impl<T: Debug> Display for Sp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use internment::LocalIntern;

#[derive(Debug, Clone, Logos, PartialEq, PartialOrd)]
pub enum Tokens {
    // Punctuation
    #[token("*")]
    Star,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Fslash,

    #[token("_")]
    Underscore,

    #[token("[")]
    Lbracket,

    #[token("]")]
    Rbracket,

    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token("{")]
    Lbrace,

    #[token("}")]
    Rbrace,

    #[token("<-")]
    Larrow,

    #[token("->")]
    Rarrow,

    #[token(";")]
    Semi,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("?")]
    Question,

    // Keywords and literals
    #[token("let")]
    Let,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[regex(r#"[a-z]?"(?:\\.|[^\\"])*""#, |lex| lex.slice().to_owned())]
    StringLiteral(String),

    #[regex(r"([0-9][0-9_]*(\.[0-9_]+)?)", |lex| lex.slice().parse())]
    Number(f64),

    #[regex("0b[01](_?[01]+)*")]
    BinaryLiteral,

    #[regex("0x[a-fA-F0-9](_?[a-fA-F0-9]+)*")]
    HexLiteral,

    #[regex("0o[0-7](_?[0-7]+)*")]
    OctalLiteral,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| LocalIntern::new(lex.slice().to_owned()))]
    Ident(LocalIntern<String>),

    #[token("\n")]
    Newline,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn tokens() {
        dbg!(Lexer::new(r#"[a, b, c] -> [c, b, a]"#, None,).parse());
    }
}
