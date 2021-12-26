use crate::ast::{expr, stmt};

pub struct TreePrinter {
    prefix: &'static str,
    level: usize,
}

impl TreePrinter {
    pub fn print_stmt(&mut self, stmt: &stmt::Stmt) {
        use stmt::StmtKind::*;
        match &stmt.kind {
            Var(var) => {
                self.emit("Var Decl");
                self.nest(|s| {
                    s.emit(format!("Name = `{}`", var.name));
                    if let Some(init) = &var.init {
                        s.emit("Var Init");
                        s.nest(|s| s.print_expr(init));
                    }
                });
            }
            Print(print) => {
                self.emit("Print Stmt");
                self.nest(|s| {
                    s.print_expr(&print.expr);
                });
            }
            Expr(expr) => {
                self.emit("Expr Stmt");
                self.nest(|s| {
                    s.print_expr(&expr.expr);
                });
            }
        }
    }

    pub fn print_expr(&mut self, expr: &expr::Expr) {
        use expr::ExprKind::*;
        match &expr.kind {
            Lit(expr::Lit { value, .. }) => {
                self.emit(format!("Literal ({:?} :: {})", value, value.type_name()));
            }
            Var(var) => {
                self.emit(format!("Var `{}`", var.name));
            }
            Group(group) => {
                self.emit("Group");
                self.nest(|s| {
                    s.print_expr(&group.expr);
                });
            }
            Unary(unary) => {
                self.emit(format!("Unary {}", unary.operator));
                self.nest(|s| {
                    s.print_expr(&unary.operand);
                });
            }
            Binary(binary) => {
                self.emit(format!("Binary {}", binary.operator));
                self.nest(|s| {
                    s.print_expr(&binary.left);
                    s.print_expr(&binary.right);
                });
            }
            Assignment(assignment) => {
                self.emit("Assignment");
                self.nest(|s| {
                    s.emit(format!("Target `{}`", assignment.name));
                    s.emit("Value");
                    s.nest(|s| s.print_expr(&assignment.value));
                });
            }
        }
    }

    pub fn new(prefix: &'static str) -> Self {
        Self { level: 0, prefix }
    }

    fn emit(&self, str: impl Into<String>) {
        println!("{}{}{}", self.prefix, " . ".repeat(self.level), str.into());
    }

    fn nest<S: FnOnce(&mut Self)>(&mut self, scope: S) {
        self.level += 1;
        scope(self);
        self.level -= 1;
    }
}
