use crate::{
    ast::{
        expr::{self, Expr, ExprKind},
        stmt::{self, Stmt, StmtKind},
    },
    interpreter::error::RuntimeError,
    token::TokenKind,
    value::LoxValue,
};

pub mod error;

type IResult<T> = Result<T, RuntimeError>;

macro_rules! bin_op_num {
    ( $left:tt $op:tt $right:tt, $op_token:expr ) => {
        match ($left, $right) {
            (Number(left), Number(right)) => Ok(Number(left $op right)),
            (left, right) => Err(RuntimeError::UnsupportedType {
                message: format!(
                    "Binary `{}` operator can only operate over two numbers. \
                    Got types `{}` and `{}`",
                    stringify!($op),
                    left.type_name(),
                    right.type_name()
                ),
                operation_span: $op_token.span
            }),
        }
    };
}

macro_rules! bin_op_cmp {
    ( $left:tt $op:tt $right:tt, $op_token:expr ) => {
        match ($left, $right) {
            (Number(left), Number(right)) => Ok(LoxValue::Boolean(left $op right)),
            (String(left), String(right)) => Ok(LoxValue::Boolean(left $op right)),
            (left, right) => Err(RuntimeError::UnsupportedType {
                message: format!(
                    "Binary `{}` operator can only compare two numbers or two strings. \
                    Got types `{}` and `{}`",
                    stringify!($op),
                    left.type_name(),
                    right.type_name()
                ),
                operation_span: $op_token.span,
            }),
        }
    };
}

pub struct Interpreter;

// The interpreter implementation.
impl Interpreter {
    pub fn interpret(&mut self, stmts: &[Stmt]) -> IResult<()> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    //
    // Statements
    //

    fn eval_stmt(&mut self, stmt: &Stmt) -> IResult<()> {
        use StmtKind::*;
        match &stmt.kind {
            Expr(expr) => self.eval_expr_stmt(expr),
            Print(print) => self.eval_print_stmt(print),
        }
    }

    fn eval_print_stmt(&mut self, stmt: &stmt::Print) -> IResult<()> {
        let val = self.eval_expr(&stmt.expr)?;
        println!("{}", val);
        Ok(())
    }

    fn eval_expr_stmt(&mut self, stmt: &stmt::Expr) -> IResult<()> {
        self.eval_expr(&stmt.expr)?;
        Ok(())
    }

    //
    // Expressions
    //

    fn eval_expr(&mut self, expr: &Expr) -> IResult<LoxValue> {
        use ExprKind::*;
        match &expr.kind {
            Literal(literal) => self.eval_literal_expr(literal),
            Group(group) => self.eval_group_expr(group),
            Unary(unary) => self.eval_unary_expr(unary),
            Binary(binary) => self.eval_binary_expr(binary),
        }
    }

    fn eval_literal_expr(&mut self, lit: &expr::Literal) -> IResult<LoxValue> {
        Ok(lit.value.clone())
    }

    fn eval_group_expr(&mut self, group: &expr::Group) -> IResult<LoxValue> {
        self.eval_expr(&group.expr)
    }

    fn eval_unary_expr(&mut self, unary: &expr::Unary) -> IResult<LoxValue> {
        let operand = self.eval_expr(&unary.operand)?;
        match &unary.operator.kind {
            TokenKind::Minus => match operand {
                LoxValue::Number(number) => Ok(LoxValue::Number(-number)),
                unexpected => Err(RuntimeError::UnsupportedType {
                    message: format!(
                        "Bad type for unary `-` operator: `{}`",
                        unexpected.type_name()
                    ),
                    operation_span: unary.operator.span,
                }),
            },
            TokenKind::Bang => Ok(LoxValue::Boolean(!lox_value_to_rust_bool(operand))),
            TokenKind::Show => Ok(LoxValue::String(operand.to_string())),
            TokenKind::Typeof => Ok(LoxValue::String(operand.type_name().into())),
            unexpected => unreachable!("Invalid unary operator ({:?}).", unexpected),
        }
    }

    fn eval_binary_expr(&mut self, binary: &expr::Binary) -> IResult<LoxValue> {
        use LoxValue::*;
        let left = self.eval_expr(&binary.left)?;
        let right = self.eval_expr(&binary.right)?;
        match &binary.operator.kind {
            TokenKind::Plus => match (left, right) {
                (Number(left), Number(right)) => Ok(Number(left + right)),
                (String(left), String(right)) => Ok(String(left + &right)),
                (left, right) => Err(RuntimeError::UnsupportedType {
                    message: format!(
                        "Binary `+` operator can only operate over two numbers or two strings. \
                        Got types `{}` and `{}`",
                        left.type_name(),
                        right.type_name()
                    ),
                    operation_span: binary.operator.span,
                }),
            },

            TokenKind::Minus => bin_op_num!(left - right, binary.operator),
            TokenKind::Star => bin_op_num!(left * right, binary.operator),
            TokenKind::Slash => {
                if let Number(right_num) = right {
                    if right_num == 0.0 {
                        return Err(RuntimeError::ZeroDivision {
                            operation_span: binary.operator.span,
                        });
                    }
                }
                bin_op_num!(left / right, binary.operator)
            }

            TokenKind::EqualEqual => Ok(LoxValue::Boolean(lox_value_equal(&left, &right))),
            TokenKind::BangEqual => Ok(LoxValue::Boolean(!lox_value_equal(&left, &right))),

            TokenKind::Greater => bin_op_cmp!(left > right, binary.operator),
            TokenKind::GreaterEqual => bin_op_cmp!(left >= right, binary.operator),
            TokenKind::Less => bin_op_cmp!(left < right, binary.operator),
            TokenKind::LessEqual => bin_op_cmp!(left <= right, binary.operator),

            unexpected => unreachable!("Invalid binary operator ({:?}).", unexpected),
        }
    }
}

//
// Some other utilities.
//

/// Tries to convert a `LoxValue` to a Rust bool.
///   * Truthy lox values: all numbers (incl. 0), all strings (incl. "") and `true`.
///   * Falsy lox values: `false` and `nil`.
fn lox_value_to_rust_bool(value: LoxValue) -> bool {
    use LoxValue::*;
    match value {
        Boolean(inner) => inner,
        Number(_) | String(_) => true,
        Nil => false,
    }
}

/// Checks if two `LoxValue`s are equal. No type coercion is performed so both types must be equal.
fn lox_value_equal(a: &LoxValue, b: &LoxValue) -> bool {
    use LoxValue::*;
    match (a, b) {
        (Boolean(a), Boolean(b)) => a == b,
        (Number(a), Number(b)) => a == b,
        (String(a), String(b)) => a == b,
        (Nil, Nil) => true,
        _ => false,
    }
}
