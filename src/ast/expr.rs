use std::fmt::{self, Display};

use crate::{
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

make_enum!(ExprKind, [Literal, Group, Unary, Binary]);

impl ExprKind {
    /// Converts the `ExprKind` into an `Expr` given a span.
    pub fn into_expr(self, span: Span) -> Expr {
        Expr { kind: self, span }
    }
}

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Literal::*;
        match self {
            Boolean(b) => write!(f, "{}", b),
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "\"{}\"", s),
            Nil => write!(f, "nil"),
        }
    }
}

impl From<TokenKind> for Literal {
    fn from(kind: TokenKind) -> Self {
        use TokenKind as T;
        match kind {
            T::String(inner) => Literal::String(inner),
            T::Number(inner) => Literal::Number(inner),
            T::False => Literal::Boolean(false),
            T::True => Literal::Boolean(true),
            T::Nil => Literal::Nil,
            _ => panic!("Invalid `Token` to `Literal` conversion. This is a bug."),
        }
    }
}

#[derive(Debug)]
pub struct Group {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub operand: Box<Expr>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
