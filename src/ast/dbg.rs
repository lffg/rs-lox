use crate::ast::{expr, stmt};

pub struct TreePrinter {
    prefix: &'static str,
    level: usize,
}

impl TreePrinter {
    pub fn print_stmt(&mut self, stmt: &stmt::Stmt) {
        use stmt::StmtKind::*;
        match &stmt.kind {
            Expr(expr) => {
                self.emit("Expr Stmt");
                self.bump_print_expr(&expr.expr);
            }
            Print(print) => {
                self.emit("Print Stmt");
                self.bump_print_expr(&print.expr);
            }
        }
    }

    pub fn print_expr(&mut self, expr: &expr::Expr) {
        use expr::ExprKind::*;
        match &expr.kind {
            Lit(expr::Lit { value, .. }) => {
                self.emit(format!("Literal ({} :: {})", value, value.type_name()));
            }
            Group(group) => {
                self.emit("Group");
                self.bump_print_expr(&group.expr);
            }
            Unary(unary) => {
                self.emit("Unary {}");
                self.bump_print_expr(&unary.operand);
            }
            Binary(binary) => {
                self.emit("Binary {}");
                self.bump_print_expr(&binary.left);
                self.bump_print_expr(&binary.right);
            }
        }
    }

    fn _bump_print_stmt(&mut self, stmt: &stmt::Stmt) {
        self.level += 1;
        self.print_stmt(stmt);
    }

    fn bump_print_expr(&mut self, expr: &expr::Expr) {
        self.level += 1;
        self.print_expr(expr);
    }

    pub fn new(prefix: &'static str) -> Self {
        Self { level: 0, prefix }
    }

    fn emit(&self, str: impl Into<String>) {
        println!("{}{}{}", self.prefix, " . ".repeat(self.level), str.into());
    }
}
