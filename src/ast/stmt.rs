use crate::{ast::expr::Expr as AstExpr, span::Span};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

make_ast_enum!(StmtKind, [Expr, Print]);

#[derive(Debug)]
pub struct Expr {
    pub expr: AstExpr,
}

#[derive(Debug)]
pub struct Print {
    pub expr: AstExpr,
}
