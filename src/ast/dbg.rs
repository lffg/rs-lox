use crate::{
    ast::{expr, stmt},
    parser::scanner::Scanner,
};

/// Prints the scanner result for the given source.
/// Will scan the entire source string and discard the result.
pub fn print_scanned_tokens(src: &str) {
    let scanner = Scanner::new(src);
    println!("┌─");
    for token in scanner {
        println!("│ {:?}", token);
    }
    println!("└─");
}

/// Prints the given program tree.
pub fn print_program_tree(stmts: &[stmt::Stmt]) {
    println!("┌─");
    TreePrinter::new("│ ").print_stmts(stmts);
    println!("└─");
}

struct TreePrinter {
    prefix: &'static str,
    level: usize,
}

impl TreePrinter {
    fn print_stmts(&mut self, stmts: &[stmt::Stmt]) {
        for (i, stmt) in stmts.iter().enumerate() {
            self.print_stmt(stmt);
            if i != stmts.len() - 1 {
                self.emit("");
            }
        }
    }

    fn print_stmt(&mut self, stmt: &stmt::Stmt) {
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
            If(if_stmt) => {
                self.emit("If Stmt");
                self.nest(|s| {
                    s.emit("Cond Expr");
                    s.nest(|s| s.print_expr(&if_stmt.cond));
                    s.emit("Then");
                    s.nest(|s| s.print_stmt(&if_stmt.then_branch));
                    if let Some(else_branch) = &if_stmt.else_branch {
                        s.emit("Else");
                        s.nest(|s| s.print_stmt(else_branch));
                    }
                })
            }
            While(while_stmt) => {
                self.emit("While Stmt");
                self.nest(|s| {
                    s.emit("Cond Expr");
                    s.nest(|s| s.print_expr(&while_stmt.cond));
                    s.emit("Body");
                    s.nest(|s| s.print_stmt(&while_stmt.body));
                })
            }
            Print(print) => {
                self.emit("Print Stmt");
                self.nest(|s| {
                    s.print_expr(&print.expr);
                });
            }
            Block(block) => {
                self.emit("Block Stmt");
                self.nest(|s| s.print_stmts(&block.stmts));
            }
            Expr(expr) => {
                self.emit("Expr Stmt");
                self.nest(|s| {
                    s.print_expr(&expr.expr);
                });
            }
            Dummy(_) => self.emit("Dummy Stmt (INVALID TREE)"),
        }
    }

    fn print_expr(&mut self, expr: &expr::Expr) {
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
            #[rustfmt::skip]
            Binary(expr::Binary { operator, left, right }) |
            Logical(expr::Logical { operator, left, right }) => {
                self.emit(format!("Binary {}", operator));
                self.nest(|s| {
                    s.print_expr(left);
                    s.print_expr(right);
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

    fn new(prefix: &'static str) -> Self {
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
