use std::{
    collections::{hash_map::Entry, HashMap},
    mem,
};

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        stmt::{self, Stmt, StmtKind},
    },
    data::LoxIdent,
    interpreter::Interpreter,
    span::Span,
};

#[derive(Debug)]
pub struct Resolver<'i> {
    interpreter: &'i mut Interpreter,
    state: ResolverState,
    scopes: Vec<HashMap<String, BindingState>>,
    errors: Vec<ResolveError>,
}

impl Resolver<'_> {
    pub fn resolve(mut self, stmts: &[Stmt]) -> (bool, Vec<ResolveError>) {
        self.resolve_stmts(stmts);
        (self.errors.is_empty(), self.errors)
    }

    //
    // Statements
    //

    fn resolve_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        use StmtKind::*;
        match &stmt.kind {
            VarDecl(var) => {
                self.declare(&var.name);
                if let Some(init) = &var.init {
                    self.resolve_expr(init);
                }
                self.define(&var.name);
            }
            FunDecl(fun) => {
                self.declare(&fun.name);
                self.define(&fun.name);
                self.resolve_function(fun, FunctionKind::Function);
            }
            If(if_stmt) => {
                self.resolve_expr(&if_stmt.cond);
                self.resolve_stmt(&if_stmt.then_branch);
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.resolve_stmt(else_branch);
                }
            }
            While(while_stmt) => {
                self.resolve_expr(&while_stmt.cond);
                self.resolve_stmt(&while_stmt.body);
            }
            Return(return_stmt) => {
                if self.state.function_kind == FunctionKind::None {
                    self.error(return_stmt.return_span, "Illegal return statement");
                }
                if let Some(value) = &return_stmt.value {
                    self.resolve_expr(value);
                }
            }
            Print(print) => self.resolve_expr(&print.expr),
            Block(block) => self.scoped(|this| this.resolve_stmts(&block.stmts)),
            Expr(expr) => self.resolve_expr(&expr.expr),
            Dummy(_) => unreachable!(),
        }
    }

    //
    // Expressions
    //

    fn resolve_expr(&mut self, expr: &Expr) {
        use ExprKind::*;
        match &expr.kind {
            Lit(_) => (),
            Var(var) => {
                if self.query(&var.name, BindingState::Declared) {
                    self.error(
                        var.name.span,
                        "Can't read local variable in its own initializer",
                    );
                    return;
                }
                self.resolve_binding(&var.name);
            }
            Group(group) => self.resolve_expr(&group.expr),
            Call(call) => {
                self.resolve_expr(&call.callee);
                for arg in &call.args {
                    self.resolve_expr(arg);
                }
            }
            Unary(unary) => self.resolve_expr(&unary.operand),
            Binary(binary) => {
                self.resolve_expr(&binary.left);
                self.resolve_expr(&binary.right);
            }
            Logical(logical) => {
                self.resolve_expr(&logical.left);
                self.resolve_expr(&logical.right);
            }
            Assignment(assignment) => {
                self.resolve_expr(&assignment.value);
                self.resolve_binding(&assignment.name);
            }
        }
    }
}

impl<'i> Resolver<'i> {
    pub fn new(interpreter: &'i mut Interpreter) -> Resolver<'i> {
        Self {
            interpreter,
            state: ResolverState::default(),
            scopes: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn declare(&mut self, ident: &LoxIdent) {
        if let Some(top) = self.scopes.last_mut() {
            let entry = top.entry(ident.name.clone());
            match entry {
                Entry::Vacant(entry) => {
                    entry.insert(BindingState::Declared);
                }
                Entry::Occupied(_) => {
                    self.error(ident.span, "Can't shadow a identifier in the same scope")
                }
            }
        }
    }

    fn define(&mut self, ident: &LoxIdent) {
        if let Some(top) = self.scopes.last_mut() {
            match top.get_mut(&ident.name) {
                Some(binding) => *binding = BindingState::Initialized,
                None => {
                    self.error(
                        ident.span,
                        format!("Binding `{}` is not defined", ident.name),
                    );
                }
            }
        }
    }

    fn query(&mut self, ident: &LoxIdent, expected: BindingState) -> bool {
        self.scopes.last().and_then(|scope| scope.get(&ident.name)) == Some(&expected)
    }

    fn resolve_binding(&mut self, ident: &LoxIdent) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&ident.name) {
                self.interpreter.resolve_local(ident, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, decl: &stmt::FunDecl, kind: FunctionKind) {
        let old = mem::replace(&mut self.state.function_kind, kind);
        self.scoped(|this| {
            for param in &decl.params {
                this.declare(param);
                this.define(param);
            }
            this.resolve_stmts(&decl.body);
        });
        self.state.function_kind = old;
    }

    fn scoped<I>(&mut self, inner: I)
    where
        I: FnOnce(&mut Self),
    {
        self.scopes.push(HashMap::new());
        let res = inner(self);
        self.scopes.pop();
        res
    }

    fn error(&mut self, span: Span, message: impl Into<String>) {
        let message = message.into();
        self.errors.push(ResolveError { span, message });
    }
}

#[derive(Debug, Default)]
struct ResolverState {
    function_kind: FunctionKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FunctionKind {
    None,
    Function,
}

impl Default for FunctionKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BindingState {
    Declared,
    Initialized,
}

#[derive(Debug)]
pub struct ResolveError {
    pub message: String,
    pub span: Span,
}
