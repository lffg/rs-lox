use crate::{
    data::{LoxIdent, LoxValue},
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

make_ast_enum!(
    ExprKind,
    [Lit, Var, Group, Call, Unary, Binary, Logical, Assignment]
);

#[derive(Debug, Clone)]
pub struct Lit {
    pub value: LoxValue,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: LoxIdent,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
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
