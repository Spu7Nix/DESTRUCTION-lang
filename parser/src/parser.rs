use logos::Logos;
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

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

    pub fn parse(&mut self) {
        while let Some(token) = self.next_token() {
            println!("{:?} {token}", self.pos);
        }
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

#[derive(Debug, Clone, Copy, Logos, PartialEq, PartialOrd)]
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

    #[regex(r#"[a-z]?"(?:\\.|[^\\"])*""#)]
    StringLiteral,

    #[regex(r"([0-9][0-9_]*(\.[0-9_]+)?)")]
    Number,

    #[regex("0b[01](_?[01]+)*")]
    BinaryLiteral,

    #[regex("0x[a-fA-F0-9](_?[a-fA-F0-9]+)*")]
    HexLiteral,

    #[regex("0o[0-7](_?[0-7]+)*")]
    OctalLiteral,
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]+")]
    Ident,

    #[token("\n")]
    Newline,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,  
}