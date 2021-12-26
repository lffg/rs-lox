use crate::{
    ast::{
        expr::{self, Expr, ExprKind},
        stmt::{self, Stmt, StmtKind},
    },
    interpreter::{environment::Environment, error::RuntimeError},
    token::TokenKind,
    value::LoxValue,
};

mod environment;
pub mod error;

type IResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub struct Interpreter {
    env: Environment,
}

// The interpreter implementation.
impl Interpreter {
    pub fn new() -> Self {
        let env = Environment::new();
        Self { env }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> IResult<()> {
        self.eval_stmts(stmts)
    }

    //
    // Statements
    //

    fn eval_stmts(&mut self, stmts: &[Stmt]) -> IResult<()> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> IResult<()> {
        use StmtKind::*;
        match &stmt.kind {
            Var(var) => self.eval_var_stmt(var),
            Print(print) => self.eval_print_stmt(print),
            Block(block) => self.eval_block_stmt(block),
            Expr(expr) => self.eval_expr(&expr.expr).map(drop),
            Dummy(_) => unreachable!(),
        }
    }

    fn eval_var_stmt(&mut self, var: &stmt::Var) -> IResult<()> {
        let value = match var.init {
            Some(ref expr) => self.eval_expr(expr)?,
            None => LoxValue::Nil,
        };
        self.env.define(var.name.clone(), value);
        Ok(())
    }

    fn eval_print_stmt(&mut self, print: &stmt::Print) -> IResult<()> {
        let val = self.eval_expr(&print.expr)?;
        match print.debug {
            true => println!("{:?}", val),
            false => println!("{}", val),
        }
        Ok(())
    }

    // What Rust allows me:
    fn eval_block_stmt(&mut self, block: &stmt::Block) -> IResult<()> {
        self.env.add_scope();
        let res = self.eval_stmts(&block.stmts);
        self.env.pop_scope();
        res
    }

    //
    // Expressions
    //

    fn eval_expr(&mut self, expr: &Expr) -> IResult<LoxValue> {
        use ExprKind::*;
        match &expr.kind {
            Lit(lit) => self.eval_lit_expr(lit),
            Var(var) => self.env.read(&var.name),
            Group(group) => self.eval_group_expr(group),
            Unary(unary) => self.eval_unary_expr(unary),
            Binary(binary) => self.eval_binary_expr(binary),
            Assignment(assignment) => self.eval_assignment_expr(assignment),
        }
    }

    fn eval_lit_expr(&mut self, lit: &expr::Lit) -> IResult<LoxValue> {
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

            TokenKind::Minus => bin_number_operator!(left - right, binary.operator),
            TokenKind::Star => bin_number_operator!(left * right, binary.operator),
            TokenKind::Slash => {
                if let Number(right_num) = right {
                    if right_num == 0.0 {
                        return Err(RuntimeError::ZeroDivision {
                            operation_span: binary.operator.span,
                        });
                    }
                }
                bin_number_operator!(left / right, binary.operator)
            }

            TokenKind::EqualEqual => Ok(LoxValue::Boolean(lox_value_equal(&left, &right))),
            TokenKind::BangEqual => Ok(LoxValue::Boolean(!lox_value_equal(&left, &right))),

            TokenKind::Greater => bin_comparison_operator!(left > right, binary.operator),
            TokenKind::GreaterEqual => bin_comparison_operator!(left >= right, binary.operator),
            TokenKind::Less => bin_comparison_operator!(left < right, binary.operator),
            TokenKind::LessEqual => bin_comparison_operator!(left <= right, binary.operator),

            unexpected => unreachable!("Invalid binary operator ({:?}).", unexpected),
        }
    }

    fn eval_assignment_expr(&mut self, assignment: &expr::Assignment) -> IResult<LoxValue> {
        let value = self.eval_expr(&assignment.value)?;
        self.env.assign(&assignment.name, value)
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

macro_rules! bin_number_operator {
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
use bin_number_operator;

macro_rules! bin_comparison_operator {
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
use bin_comparison_operator;
