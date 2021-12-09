use crate::span::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexme: String, // TODO: Change to interned string symbol.
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexme: &str, span: Span) -> Self {
        let lexme = lexme.into();
        Token { kind, lexme, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
