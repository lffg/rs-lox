use std::fmt::{self, Display};

use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    String(String),
    Number(f64),

    Comment(String),
    NewLine,
    Whitespace,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Comma,
    Semicolon,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Nil,
    True,
    False,
    This,
    Super,
    Class,
    And,
    Or,
    If,
    Else,
    Return,
    Fun,
    For,
    While,
    Var,
    Print,

    Eof,

    Dummy,
    Error(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match &self.kind {
            Identifier(s) | String(s) => write!(f, "{}", s),
            Number(n) => write!(f, "{}", n),
            Comment(s) => write!(f, "//{}", s),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Dot => write!(f, "."),
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Nil => write!(f, "nil"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            This => write!(f, "this"),
            Super => write!(f, "super"),
            Class => write!(f, "class"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Return => write!(f, "return"),
            Fun => write!(f, "fun"),
            For => write!(f, "for"),
            While => write!(f, "while"),
            Var => write!(f, "var"),
            Print => write!(f, "print"),

            NewLine => write!(f, "<newline>"),
            Whitespace => write!(f, "<whitespace>"),
            Eof => write!(f, "<eof>"),
            Dummy => write!(f, "<dummy>"),
            Error(e) => write!(f, "<error: {}>", e),
        }
    }
}
