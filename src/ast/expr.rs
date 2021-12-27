use crate::{
    span::Span,
    token::{Token, TokenKind},
    value::LoxValue,
};

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

make_ast_enum!(
    ExprKind,
    [Lit, Var, Group, Unary, Binary, Logical, Assignment]
);

#[derive(Debug)]
pub struct Lit {
    pub value: LoxValue,
}

#[derive(Debug)]
pub struct Var {
    pub name: String,
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

#[derive(Debug)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Assignment {
    pub name: String,
    pub name_span: Span,
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
