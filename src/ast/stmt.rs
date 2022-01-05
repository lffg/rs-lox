use crate::{
    ast::{expr, AstId},
    data::LoxIdent,
    span::Span,
};

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
    pub id: AstId,
}

impl Stmt {
    pub fn new(span: Span, kind: impl Into<StmtKind>) -> Self {
        Self {
            span,
            kind: kind.into(),
            id: AstId::new(),
        }
    }
}

make_ast_enum!(
    StmtKind,
    [VarDecl, FunDecl, If, While, Return, Print, Block, Expr, Dummy]
);

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: LoxIdent,
    pub init: Option<expr::Expr>,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: LoxIdent,
    pub params: Vec<LoxIdent>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: expr::Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub cond: expr::Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub return_span: Span,
    pub value: Option<expr::Expr>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expr: expr::Expr,
    pub debug: bool,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub expr: expr::Expr,
}

/// For error purposes.
#[derive(Debug, Clone)]
pub struct Dummy();
