pub mod tree {
    use crate::ast::expr::{Expr, ExprKind::*};

    pub fn print_expr(expr: &Expr) {
        print_rec("Expr (phantom node)", [expr], "");
    }

    fn print_rec<const N: usize>(name: &str, exprs: [&Expr; N], indent: &str) {
        println!("{} {}", indent, name);
        let indent = &format!("{}    ", indent);
        for expr in exprs {
            match &expr.kind {
                Literal(e) => print_rec(&format!("Literal ({})", e), [], indent),
                Group(e) => print_rec(&format!("Group"), [&e.expr], indent),
                Unary(e) => print_rec(&format!("Unary (op: {})", e.operator), [&e.operand], indent),
                Binary(e) => print_rec(
                    &format!("Binary (op: {})", e.operator),
                    [&e.left, &e.right],
                    indent,
                ),
            }
        }
    }
}
