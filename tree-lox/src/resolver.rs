use std::{
    collections::{hash_map::Entry, HashMap},
    mem,
};

use crate::{
    ast::{
        expr::Expr,
        stmt::{self, Stmt},
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
        use Stmt::*;
        match &stmt {
            VarDecl(var) => {
                self.declare(&var.name);
                if let Some(init) = &var.init {
                    self.resolve_expr(init);
                }
                self.define(&var.name);
            }
            ClassDecl(class) => {
                let old_class_state = mem::replace(&mut self.state.class, ClassState::Class);

                self.declare(&class.name);
                self.define(&class.name);

                if let Some(super_name) = &class.super_name {
                    if class.name.name == super_name.name {
                        self.error(super_name.span, "Class can't inherit itself");
                    }

                    self.state.class = ClassState::SubClass;

                    self.resolve_binding(super_name);

                    // If there is a super-class, we initialize a new scope and define `super` in
                    // it so the current class may access it.
                    self.begin_scope();
                    self.initialize("super");
                }

                self.scoped(|this| {
                    this.initialize("this");
                    for method in &class.methods {
                        let state = if method.name.name == "init" {
                            FunctionState::Init
                        } else {
                            FunctionState::Method
                        };
                        this.resolve_function(method, state);
                    }
                });

                if class.super_name.is_some() {
                    self.end_scope();
                }

                self.state.class = old_class_state;
            }
            FunDecl(fun) => {
                self.declare(&fun.name);
                self.define(&fun.name);
                self.resolve_function(fun, FunctionState::Function);
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
                if self.state.function == FunctionState::None {
                    self.error(return_stmt.return_span, "Illegal return statement");
                }
                if let Some(value) = &return_stmt.value {
                    if self.state.function == FunctionState::Init {
                        self.error(
                            return_stmt.return_span,
                            "Can't return value from class initializer",
                        );
                    }
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
        use Expr::*;
        match &expr {
            Lit(_) => (),
            This(this) => {
                if self.state.class == ClassState::None {
                    self.error(
                        this.span,
                        "Illegal this expression, can't use this outside of a class",
                    );
                }
                self.resolve_binding(&this.name)
            }
            Super(sup) => {
                if self.state.class == ClassState::None {
                    self.error(
                        sup.super_ident.span,
                        "Illegal super expression, can't use super outside of a class",
                    )
                }
                if self.state.class == ClassState::Class {
                    self.error(
                        sup.super_ident.span,
                        "Illegal super expression, can't use super within a class with no superclass",
                    )
                }
                self.resolve_binding(&sup.super_ident);
            }
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
            Get(get) => {
                // Since properties are looked up dynamically by the interpreter (in a similar
                // manner to how global variables are handled), the resolver don't need to touch
                // their names.
                self.resolve_expr(&get.object);
            }
            Set(set) => {
                // Like get, the resolver doesn't need to resolve the set property name since it is
                // dynamically looked up by the interpreter.
                self.resolve_expr(&set.object);
                self.resolve_expr(&set.value);
            }
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

    fn initialize(&mut self, ident: impl Into<String>) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(ident.into(), BindingState::Initialized);
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

    fn resolve_function(&mut self, decl: &stmt::FunDecl, state: FunctionState) {
        let old_function_state = mem::replace(&mut self.state.function, state);

        self.scoped(|this| {
            for param in &decl.params {
                this.declare(param);
                this.define(param);
            }
            this.resolve_stmts(&decl.body);
        });

        self.state.function = old_function_state;
    }

    /// One should ideally use `scoped`. Callers of `begin_scope` must also call `end_scope`.
    #[inline]
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    #[inline]
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn scoped<I>(&mut self, inner: I)
    where
        I: FnOnce(&mut Self),
    {
        self.begin_scope();
        let res = inner(self);
        self.end_scope();
        res
    }

    fn error(&mut self, span: Span, message: impl Into<String>) {
        let message = message.into();
        self.errors.push(ResolveError { span, message });
    }
}

#[derive(Debug, Default)]
struct ResolverState {
    function: FunctionState,
    class: ClassState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FunctionState {
    None,
    Init,   // Class init
    Method, // Class method
    Function,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ClassState {
    None,
    Class,
    SubClass,
}

macro_rules! impl_default_for_state {
    ($($name:ident),+) => {
        $(
            impl Default for $name {
                fn default() -> Self {
                    Self::None
                }
            }
        )+
    }
}

impl_default_for_state!(FunctionState, ClassState);

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
