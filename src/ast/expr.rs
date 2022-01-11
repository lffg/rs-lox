use crate::{
    data::{LoxIdent, LoxValue},
    span::Span,
    token::{Token, TokenKind},
};

make_ast_enum!(
    Expr,
    [Lit, This, Var, Group, Super, Get, Set, Call, Unary, Binary, Logical, Assignment]
);

#[derive(Debug, Clone)]
pub struct Lit {
    pub span: Span,
    pub value: LoxValue,
}

#[derive(Debug, Clone)]
pub struct This {
    pub span: Span,
    pub name: LoxIdent,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub span: Span,
    pub name: LoxIdent,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub span: Span,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Super {
    pub span: Span,
    pub super_ident: LoxIdent,
    pub method: LoxIdent,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub span: Span,
    pub object: Box<Expr>,
    pub name: LoxIdent,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub span: Span,
    pub object: Box<Expr>,
    pub name: LoxIdent,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub span: Span,
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub span: Span,
    pub operator: Token,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub span: Span,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub span: Span,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub span: Span,
    pub name: LoxIdent,
    pub value: Box<Expr>,
}

//
// Some other utilities.
//

impl From<Token> for Lit {
    fn from(token: Token) -> Self {
        use LoxValue as L;
        use TokenKind as T;
        Lit {
            span: token.span,
            value: match token.kind {
                T::String(string) => L::String(string),
                T::Number(number) => L::Number(number),
                T::Nil => L::Nil,
                T::True => L::Boolean(true),
                T::False => L::Boolean(false),
                unexpected => unreachable!(
                    "Invalid `Token` ({:?}) to `Literal` conversion.",
                    unexpected
                ),
            },
        }
    }
}
