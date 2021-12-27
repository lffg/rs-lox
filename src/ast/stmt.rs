use crate::{ast::expr, span::Span};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

make_ast_enum!(StmtKind, [Var, If, Print, Block, Expr, Dummy]);

#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub name_span: Span,
    pub init: Option<expr::Expr>,
}

#[derive(Debug)]
pub struct If {
    pub cond: expr::Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct Print {
    pub expr: expr::Expr,
    pub debug: bool,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Expr {
    pub expr: expr::Expr,
}

/// For error purposes.
#[derive(Debug)]
pub struct Dummy();
