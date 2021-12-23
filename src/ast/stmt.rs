use crate::{ast::expr, span::Span};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

make_ast_enum!(StmtKind, [Expr, Print]);

#[derive(Debug)]
pub struct Expr {
    pub expr: expr::Expr,
}

#[derive(Debug)]
pub struct Print {
    pub expr: expr::Expr,
    pub debug: bool,
}
