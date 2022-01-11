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
            VarDecl(var) => {
                self.emit("Var Decl");
                self.nest(|s| {
                    s.emit(format!("Name = `{}`", var.name));
                    if let Some(init) = &var.init {
                        s.emit("Var Init");
                        s.nest(|s| s.print_expr(init));
                    }
                });
            }
            ClassDecl(class) => {
                self.emit("Class Decl");
                self.nest(|s| {
                    s.emit(format!("Name = `{}`", class.name));
                    if let Some(super_name) = &class.super_name {
                        s.emit(format!("Extending `{}`", super_name));
                    }
                    s.emit("Methods");
                    s.nest(|s| {
                        for method in &class.methods {
                            s.print_fun(method, "Class Method");
                        }
                    });
                });
            }
            FunDecl(fun) => self.print_fun(fun, "Fun Stmt"),
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
            Return(return_stmt) => {
                self.emit("Return Stmt");
                if let Some(value) = &return_stmt.value {
                    self.nest(|s| s.print_expr(value));
                }
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
            This(_) => {
                self.emit("This");
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
            Get(get) => {
                self.emit("Get");
                self.nest(|s| {
                    s.emit(format!("Property: `{}`", get.name));
                    s.emit("From Object");
                    s.nest(|s| s.print_expr(&get.object));
                });
            }
            Set(set) => {
                self.emit("Set");
                self.nest(|s| {
                    s.emit(format!("Target property: `{}`", set.name));
                    s.emit("From Object");
                    s.nest(|s| s.print_expr(&set.object));
                    s.emit("With Value");
                    s.nest(|s| s.print_expr(&set.value));
                });
            }
            Call(call) => {
                self.emit("Call");
                self.nest(|s| {
                    s.emit("Callee");
                    s.nest(|s| s.print_expr(&call.callee));
                    if !call.args.is_empty() {
                        s.emit("Args");
                        s.nest(|s| {
                            for arg in &call.args {
                                s.print_expr(arg);
                            }
                        });
                    }
                })
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
                    s.emit(format!("Target: `{}`", assignment.name));
                    s.emit("With Value");
                    s.nest(|s| s.print_expr(&assignment.value));
                });
            }
        }
    }

    fn print_fun(&mut self, fun: &stmt::FunDecl, label: &'static str) {
        self.emit(label);
        self.nest(|s| {
            s.emit(format!("Name = `{}`", fun.name));
            s.emit(format!("Params ({})", fun.params.len()));
            s.nest(|s| {
                for param in &fun.params {
                    s.emit(&param.name);
                }
            });
            s.emit("Body");
            s.nest(|s| s.print_stmts(&fun.body));
        });
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
