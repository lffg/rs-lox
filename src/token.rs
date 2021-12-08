use crate::span::Span;

#[derive(Debug)]
pub struct Token<'s> {
    pub kind: TokenKind,
    pub lexme: &'s str,
    pub span: Span,
}

impl<'s> Token<'s> {
    pub fn new(kind: TokenKind, lexme: &'s str, span: Span) -> Self {
        Token { kind, lexme, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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

    Identifier(String),
    String(String),
    Number(f64),

    Comment(String),

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

    Whitespace(char),
    Eof,
}
