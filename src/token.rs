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

    pub fn dummy() -> Self {
        Self::new(TokenKind::Dummy, Span::new(0, 0))
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
    Typeof,
    Show,

    Eof,

    Dummy,
    Error(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            Identifier(s) => s.fmt(f),
            Number(n) => n.fmt(f),
            String(s) => write!(f, "\"{}\"", s),
            Comment(s) => write!(f, "//{}", s),
            LeftParen => f.write_str("("),
            RightParen => f.write_str(")"),
            LeftBrace => f.write_str("{{"),
            RightBrace => f.write_str("}}"),
            Plus => f.write_str("+"),
            Minus => f.write_str("-"),
            Star => f.write_str("*"),
            Slash => f.write_str("/"),
            Dot => f.write_str("."),
            Comma => f.write_str(","),
            Semicolon => f.write_str(";"),
            Bang => f.write_str("!"),
            BangEqual => f.write_str("!="),
            Equal => f.write_str("="),
            EqualEqual => f.write_str("=="),
            Less => f.write_str("<"),
            LessEqual => f.write_str("<="),
            Greater => f.write_str(">"),
            GreaterEqual => f.write_str(">="),
            Nil => f.write_str("nil"),
            True => f.write_str("true"),
            False => f.write_str("false"),
            This => f.write_str("this"),
            Super => f.write_str("super"),
            Class => f.write_str("class"),
            And => f.write_str("and"),
            Or => f.write_str("or"),
            If => f.write_str("if"),
            Else => f.write_str("else"),
            Return => f.write_str("return"),
            Fun => f.write_str("fun"),
            For => f.write_str("for"),
            While => f.write_str("while"),
            Var => f.write_str("var"),
            Print => f.write_str("print"),
            Typeof => f.write_str("typeof"),
            Show => f.write_str("show"),

            NewLine => f.write_str("<newline>"),
            Whitespace => f.write_str("<whitespace>"),
            Eof => f.write_str("<eof>"),
            Dummy => f.write_str("<dummy>"),
            Error(e) => write!(f, "<error: {}>", e),
        }
    }
}
