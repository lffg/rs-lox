use crate::ast::expr::{Expr, ExprKind::*};

pub fn print_tree(expr: &Expr, level: usize) {
    macro_rules! emit {
            ( $( $arg:tt )* ) => {
                println!("{}{}", " . ".repeat(level), format_args!($( $arg )*));
            };
        }
    match &expr.kind {
        Literal(lit) => {
            emit!("Literal ({} :: {})", lit.value, lit.value.type_name());
        }
        Group(g) => {
            emit!("Group");
            print_tree(&g.expr, level + 1);
        }
        Unary(u) => {
            emit!("Unary {}", u.operator);
            print_tree(&u.operand, level + 1);
        }
        Binary(b) => {
            emit!("Binary {}", b.operator);
            print_tree(&b.left, level + 1);
            print_tree(&b.right, level + 1);
        }
    }
}
