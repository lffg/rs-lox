use crate::{ast::expr, data::LoxIdent, span::Span};

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(span: Span, kind: impl Into<StmtKind>) -> Self {
        Self {
            span,
            kind: kind.into(),
        }
    }
}

make_ast_enum!(
    StmtKind,
    [VarDecl, ClassDecl, FunDecl, If, While, Return, Print, Block, Expr, Dummy]
);

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: LoxIdent,
    pub init: Option<expr::Expr>,
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: LoxIdent,
    pub super_name: Option<LoxIdent>,
    pub methods: Vec<FunDecl>,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: LoxIdent,
    pub params: Vec<LoxIdent>,
    pub body: Vec<Stmt>,
    /// Span of the function parameters and body. Must NOT include, for example, the `fun` token.
    pub span: Span,
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
