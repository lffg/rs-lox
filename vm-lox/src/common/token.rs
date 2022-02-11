use crate::common::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

impl Token {
    /// Creates an returns the dummy token.
    pub fn dummy() -> Token {
        Token {
            kind: TokenKind::Dummy,
            span: Span::new(0, 0),
        }
    }
}

impl From<&Token> for Span {
    fn from(token: &Token) -> Self {
        token.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    String(String),
    Number(f64),

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
