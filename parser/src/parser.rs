use crate::ast::{Expr, Operator, StringFlag, TopLevel, Transformation, Type, UnaryOperator};
use crate::error::{LangError, LangErrorT};
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
            self.pos.0 += 1; // :3
            self.next_token()
        } else {
            self.pos.1 += self.tokens.span().len();
            Some(Token {
                data: token,
                span: self.tokens.span().into(),
            })
        }
    }

    pub fn ensure_next(&mut self) -> Token {
        self.next_token()
            .unwrap_or_else(|| self.throw_error(LangErrorT::SyntaxError, "Unexpected end of input"))
    }

    pub fn parse_transforms(&mut self) -> Vec<Transformation> {
        let mut transformations = Vec::new();

        loop {
            transformations.push(self.parse_transform());
            self.expect(Tokens::Semi);
            match self.peek() {
                None => break,
                _ => (),
            };
        }
        transformations
    }

    pub fn parse(&mut self) -> TopLevel {
        let mut transformations = Vec::new();

        if self.peek().is_some() {
            transformations = self.parse_transforms();
            if let Some(t) = self.peek() {
                self.throw_error(
                    LangErrorT::SyntaxError,
                    &format!("Unexpected token: {:?}", t.data),
                );
            }
        }

        let top_level = TopLevel { transformations };

        top_level
    }

    fn parse_maths(&mut self, operator: Tokens, lhs: Expr, rhs: Expr) -> Expr {
        match operator {
            op @ (Tokens::Star | Tokens::Minus | Tokens::Plus | Tokens::Fslash) => {
                let lhs = box lhs;
                let rhs = box rhs;
                Expr::Operator(op.into(), lhs, rhs)
            }

            t => {
                println!("{:?}", t);
                todo!()
            }
        }
    }

    fn expect(&mut self, token: Tokens) -> () {
        match self.peek() {
            Some(Token { data: t, .. }) if t == token => self.next_token(),
            _ => self.throw_error(LangErrorT::SyntaxError, &format!("Expected {:?}", token)),
        };
    }

    fn parse_expr(&mut self) -> Expr {
        let unary_operator = match self.peek() {
            Some(Token {
                data: Tokens::Minus,
                ..
            }) => {
                self.next_token();
                Some(UnaryOperator::Neg)
            }
            Some(Token {
                data: Tokens::Exclamation,
                ..
            }) => {
                self.next_token();
                Some(UnaryOperator::Not)
            }
            _ => None,
        };

        let first = match self.ensure_next().data {
            Tokens::Number(n) => Expr::Number(n),
            Tokens::False => Expr::Bool(false),
            Tokens::True => Expr::Bool(true),
            Tokens::StringLiteral(mut s) => {
                let flag = s.1;
                s.0.remove(0);
                s.0.pop();
                Expr::String(s.0, flag)
            }
            Tokens::Lbracket => {
                // check for immidiate right bracket
                if let Some(Token {
                    data: Tokens::Rbracket,
                    span: _,
                }) = self.peek()
                {
                    self.next_token();
                    return Expr::Array(Vec::new());
                }

                let mut exprs = Vec::new();
                loop {
                    exprs.push(self.parse_expr());
                    match self.ensure_next().data {
                        Tokens::Comma => (),
                        Tokens::Rbracket => break,

                        token => self.throw_error(
                            LangErrorT::SyntaxError,
                            &format!("Expected tokens `]` or `,`, found {:?}", token),
                        ),
                    }
                }
                Expr::Array(exprs)
            }

            Tokens::Ident(s) => Expr::Ident(s),
            Tokens::Underscore => Expr::Any,

            Tokens::Lparen => {
                let expr = self.parse_expr();
                if let Some(Token {
                    data: Tokens::Rparen,
                    span: _,
                }) = self.peek()
                {
                    self.next_token();
                    return expr;
                }
                self.expect(Tokens::Comma);
                let mut exprs = vec![expr];
                loop {
                    exprs.push(self.parse_expr());
                    match self.ensure_next().data {
                        Tokens::Comma => (),
                        Tokens::Rparen => break,

                        token => self.throw_error(
                            LangErrorT::SyntaxError,
                            &format!("Expected tokens `)` or `,`, found {:?}", token),
                        ),
                    }
                }
                Expr::Tuple(exprs)
            }

            token => self.throw_error(
                LangErrorT::SyntaxError,
                &format!("Unexpected token: {:?}", token),
            ),
        };

        let first = if let Some(uo) = unary_operator {
            Expr::UnaryOp(uo, box first)
        } else {
            first
        };

        match self.peek() {
            Some(Token {
                data: operator @ (Tokens::Star | Tokens::Minus | Tokens::Plus | Tokens::Fslash),
                ..
            }) => {
                self.next_token();
                let rhs = self.parse_expr();
                self.parse_maths(operator, first, rhs)
            }

            Some(Token {
                data: Tokens::DoubleColon,
                ..
            }) => {
                // cast
                // v::#from ~> #to
                self.next_token();
                let from = match self.ensure_next().data {
                    Tokens::Type(s) => {
                        let mut s2 = s.to_string();
                        s2.remove(0);
                        s2.parse::<Type>().unwrap_or_else(|_| {
                            self.throw_error(
                                LangErrorT::SyntaxError,
                                &format!("{:?} is not a valid type", s),
                            )
                        })
                    }
                    token => self.throw_error(
                        LangErrorT::SyntaxError,
                        &format!("Expected type, found {:?}", token),
                    ),
                };

                self.expect(Tokens::WavyArrow);

                let to = match self.ensure_next().data {
                    Tokens::Type(s) => {
                        let mut s2 = s.to_string();
                        s2.remove(0);
                        s2.parse::<Type>().unwrap_or_else(|_| {
                            self.throw_error(
                                LangErrorT::SyntaxError,
                                &format!("{:?} is not a valid type", s2),
                            )
                        })
                    }
                    token => self.throw_error(
                        LangErrorT::SyntaxError,
                        &format!("Expected type, found {:?}", token),
                    ),
                };

                Expr::Cast(box first, to, from)
            }
            _ => first,
        }
    }

    pub fn parse_transform(&mut self) -> Transformation {
        let destruct = self.parse_expr();
        self.expect(Tokens::Rarrow);
        let construct = self.parse_expr();
        Transformation::Forced {
            destruct,
            construct,
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
        let mut token = tokens.next()?;

        while token == Tokens::Newline {
            token = tokens.next()?;
        }

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
            if token == Tokens::Newline {
                continue;
            }
            out.push(Token {
                data: token,
                span: tokens.span().into(),
            });
            idx += 1;
        }

        out
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn file(&self) -> Option<PathBuf> {
        self.file.to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

impl From<Tokens> for Expr {
    fn from(t: Tokens) -> Self {
        match t {
            // ill add the thingy thingy at Tokens ok your
            Tokens::StringLiteral(s) => Expr::String(s.0, s.1),
            Tokens::Ident(i) => Expr::Ident(i),
            Tokens::Number(n) => Expr::Number(n),

            _ => panic!(),
        }
    }
}

impl From<Tokens> for Operator {
    fn from(t: Tokens) -> Self {
        match t {
            Tokens::Minus => Operator::Sub, // fixed now?
            Tokens::Plus => Operator::Add,
            Tokens::Star => Operator::Mul, // uhhh
            Tokens::Fslash => Operator::Div,
            _ => panic!(),
        }
    }
}

use internment::LocalIntern;

#[derive(Debug, Clone, Logos, PartialEq, PartialOrd)] // push push //
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

    #[token("~>")]
    WavyArrow,

    #[token("|")]
    Pipe,

    #[token(";")]
    Semi,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("?")]
    Question,

    #[token("!")]
    Exclamation,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanEqual,

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

    #[regex(r#"[f]?"(?:\\.|[^\\"])*""#, |lex| {
        let mut s = lex.slice().to_owned();
        let flag = if !s.starts_with('"') { // well anyways theres an error up here
            match s.remove(0) { // we dont need Option because of constraints in the token regex
                'f' => Some(StringFlag::Format),
                _ => unreachable!(),
            }
        } else {
            None
        };
        (s, flag)
    })]
    StringLiteral((String, Option<StringFlag>)),

    #[regex(r"([0-9][0-9_]*(\.[0-9_]+)?)", |lex| lex.slice().parse())]
    // like here // where does the error go // Err token // a
    Number(f64),

    #[regex("0b[01](_?[01]+)*")]
    BinaryLiteral,

    #[regex("0x[a-fA-F0-9](_?[a-fA-F0-9]+)*")]
    HexLiteral,

    #[regex("0o[0-7](_?[0-7]+)*")]
    OctalLiteral,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| LocalIntern::new(lex.slice().to_owned()))]
    Ident(LocalIntern<String>),

    #[regex(r"#[a-zA-Z_][a-zA-Z0-9_]*", |lex| LocalIntern::new(lex.slice().to_owned()))]
    Type(LocalIntern<String>),

    #[token("\n")]
    Newline,

    #[error]
    #[regex(r"[ \t\f\r]+", logos::skip)]
    Error,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn tokens() {
        dbg!(Lexer::new(
            r#"
[a, b] -> [b, a]
[a, b] -> a + b
    
        "#,
            None,
        )
        .parse());
    }
}
