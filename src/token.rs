#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexme: String, // TODO: Change to an interned string symbol.
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexme: &str, line: usize) -> Self {
        let lexme = lexme.into();
        Self { kind, lexme, line }
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
}
