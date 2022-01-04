use std::{mem, rc::Rc};

use crate::{
    ast::{
        expr::{self, Expr, ExprKind},
        stmt::{self, Stmt, StmtKind},
    },
    data::{LoxFunction, LoxIdent, LoxValue, NativeFunction},
    interpreter::{control_flow::ControlFlow, environment::Environment, error::RuntimeError},
    span::Span,
    token::TokenKind,
};

pub mod control_flow;
pub mod environment;
pub mod error;

/// Control flow result
pub type CFResult<T> = Result<T, ControlFlow<LoxValue, RuntimeError>>;

#[derive(Debug)]
pub struct Interpreter {
    pub env: Environment,
}

// The interpreter implementation.
impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    // Note that `CFResult` must not be exposed to the interpreter caller.
    // It is an implementation detail.
    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        match self.eval_stmts(stmts) {
            Ok(()) => Ok(()),
            Err(ControlFlow::Err(err)) => Err(err),
            Err(ControlFlow::Return(_)) => unreachable!(),
        }
    }

    //
    // Statements
    //

    fn eval_stmts(&mut self, stmts: &[Stmt]) -> CFResult<()> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> CFResult<()> {
        use StmtKind::*;
        match &stmt.kind {
            Var(var) => self.eval_var_stmt(var),
            Fun(fun) => self.eval_fun_stmt(fun),
            If(if_stmt) => self.eval_if_stmt(if_stmt),
            While(while_stmt) => self.eval_while_stmt(while_stmt),
            Return(return_stmt) => self.eval_return_stmt(return_stmt),
            Print(print) => self.eval_print_stmt(print),
            Block(block) => self.eval_block(&block.stmts, Environment::new_enclosed(&self.env)),
            Expr(expr) => self.eval_expr(&expr.expr).map(drop),
            Dummy(_) => unreachable!(),
        }
    }

    fn eval_var_stmt(&mut self, var: &stmt::Var) -> CFResult<()> {
        let value = match &var.init {
            Some(expr) => self.eval_expr(expr)?,
            None => LoxValue::Nil,
        };
        self.env.define(var.name.clone(), value);
        Ok(())
    }

    fn eval_fun_stmt(&mut self, fun: &stmt::Fun) -> CFResult<()> {
        self.env.define(
            fun.name.clone(),
            LoxValue::Function(Rc::new(LoxFunction {
                fun_stmt: fun.clone(),
            })),
        );
        Ok(())
    }

    fn eval_if_stmt(&mut self, if_stmt: &stmt::If) -> CFResult<()> {
        let cond_value = self.eval_expr(&if_stmt.cond)?;
        if lox_is_truthy(&cond_value) {
            self.eval_stmt(&if_stmt.then_branch)?;
        } else if let Some(else_branch) = &if_stmt.else_branch {
            self.eval_stmt(else_branch)?;
        }
        Ok(())
    }

    fn eval_while_stmt(&mut self, while_stmt: &stmt::While) -> CFResult<()> {
        while lox_is_truthy(&self.eval_expr(&while_stmt.cond)?) {
            self.eval_stmt(&while_stmt.body)?;
        }
        Ok(())
    }

    fn eval_return_stmt(&mut self, return_stmt: &stmt::Return) -> CFResult<()> {
        let value = return_stmt
            .value
            .as_ref()
            .map(|expr| self.eval_expr(expr))
            .transpose()?
            .unwrap_or(LoxValue::Nil);
        Err(ControlFlow::Return(value))
    }

    fn eval_print_stmt(&mut self, print: &stmt::Print) -> CFResult<()> {
        let val = self.eval_expr(&print.expr)?;
        match print.debug {
            true => println!("{:?}", val),
            false => println!("{}", val),
        }
        Ok(())
    }

    pub(crate) fn eval_block(&mut self, stmts: &[Stmt], new_env: Environment) -> CFResult<()> {
        let old_env = mem::replace(&mut self.env, new_env);
        let result = self.eval_stmts(stmts);
        self.env = old_env;
        result
    }

    //
    // Expressions
    //

    fn eval_expr(&mut self, expr: &Expr) -> CFResult<LoxValue> {
        use ExprKind::*;
        match &expr.kind {
            Lit(lit) => self.eval_lit_expr(lit),
            Var(var) => self.env.read(&var.name),
            Group(group) => self.eval_group_expr(group),
            Call(call) => self.eval_call_expr(call, expr.span),
            Unary(unary) => self.eval_unary_expr(unary),
            Binary(binary) => self.eval_binary_expr(binary),
            Logical(logical) => self.eval_logical_expr(logical),
            Assignment(assignment) => self.eval_assignment_expr(assignment),
        }
    }

    fn eval_lit_expr(&mut self, lit: &expr::Lit) -> CFResult<LoxValue> {
        Ok(lit.value.clone())
    }

    fn eval_group_expr(&mut self, group: &expr::Group) -> CFResult<LoxValue> {
        self.eval_expr(&group.expr)
    }

    fn eval_call_expr(&mut self, call: &expr::Call, span: Span) -> CFResult<LoxValue> {
        use LoxValue::*;
        let callee = self.eval_expr(&call.callee)?;
        let args = call
            .args
            .iter()
            .map(|expr| self.eval_expr(expr))
            .collect::<Result<Vec<_>, _>>()?;

        match callee {
            Function(callable) if callable.arity() == args.len() => callable.call(self, &args),
            Function(callable) => Err(RuntimeError::UnsupportedType {
                message: format!(
                    "Expected {} arguments, but got {}",
                    callable.arity(),
                    args.len()
                ),
                span,
            }
            .into()),
            _ => Err(RuntimeError::UnsupportedType {
                message: format!(
                    "Type `{}` is not callable, can only call functions and classes",
                    callee.type_name()
                ),
                span,
            }
            .into()),
        }
    }

    fn eval_unary_expr(&mut self, unary: &expr::Unary) -> CFResult<LoxValue> {
        let operand = self.eval_expr(&unary.operand)?;
        match &unary.operator.kind {
            TokenKind::Minus => match operand {
                LoxValue::Number(number) => Ok(LoxValue::Number(-number)),
                unexpected => Err(RuntimeError::UnsupportedType {
                    message: format!(
                        "Bad type for unary `-` operator: `{}`",
                        unexpected.type_name()
                    ),
                    span: unary.operator.span,
                }
                .into()),
            },
            TokenKind::Bang => Ok(LoxValue::Boolean(!lox_is_truthy(&operand))),
            TokenKind::Show => Ok(LoxValue::String(operand.to_string())),
            TokenKind::Typeof => Ok(LoxValue::String(operand.type_name().into())),
            unexpected => unreachable!("Invalid unary operator ({:?}).", unexpected),
        }
    }

    fn eval_binary_expr(&mut self, binary: &expr::Binary) -> CFResult<LoxValue> {
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
                    span: binary.operator.span,
                }
                .into()),
            },

            TokenKind::Minus => bin_number_operator!(left - right, binary.operator),
            TokenKind::Star => bin_number_operator!(left * right, binary.operator),
            TokenKind::Slash => {
                if let Number(right_num) = right {
                    if right_num == 0.0 {
                        return Err(RuntimeError::ZeroDivision {
                            span: binary.operator.span,
                        }
                        .into());
                    }
                }
                bin_number_operator!(left / right, binary.operator)
            }

            TokenKind::EqualEqual => Ok(LoxValue::Boolean(lox_is_equal(&left, &right))),
            TokenKind::BangEqual => Ok(LoxValue::Boolean(!lox_is_equal(&left, &right))),

            TokenKind::Greater => bin_comparison_operator!(left > right, binary.operator),
            TokenKind::GreaterEqual => bin_comparison_operator!(left >= right, binary.operator),
            TokenKind::Less => bin_comparison_operator!(left < right, binary.operator),
            TokenKind::LessEqual => bin_comparison_operator!(left <= right, binary.operator),

            unexpected => unreachable!("Invalid binary operator ({:?}).", unexpected),
        }
    }

    fn eval_logical_expr(&mut self, logical: &expr::Logical) -> CFResult<LoxValue> {
        let left = self.eval_expr(&logical.left)?;
        match &logical.operator.kind {
            TokenKind::And if !lox_is_truthy(&left) => Ok(left),
            TokenKind::Or if lox_is_truthy(&left) => Ok(left),
            _ => self.eval_expr(&logical.right),
        }
    }

    fn eval_assignment_expr(&mut self, assignment: &expr::Assignment) -> CFResult<LoxValue> {
        let value = self.eval_expr(&assignment.value)?;
        self.env.assign(&assignment.name, value)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut global_env = Environment::new();

        def_native!(
            global_env.clock / 0,
            fn clock(_: &[LoxValue]) -> CFResult<LoxValue> {
                use std::time::{SystemTime, UNIX_EPOCH};
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                Ok(LoxValue::Number(since_the_epoch))
            }
        );

        Self { env: global_env }
    }
}

//
// Some other utilities.
//

/// Tries to convert a `LoxValue` to a Rust bool.
///   * Truthy lox values: all numbers (incl. 0), all strings (incl. "") and `true`.
///   * Falsy lox values: `false` and `nil`.
fn lox_is_truthy(value: &LoxValue) -> bool {
    use LoxValue::*;
    match value {
        Boolean(inner) => *inner,
        Function(_) | Number(_) | String(_) => true,
        Nil => false,
    }
}

/// Checks if two `LoxValue`s are equal. No type coercion is performed so both types must be equal.
fn lox_is_equal(a: &LoxValue, b: &LoxValue) -> bool {
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
                span: $op_token.span
            }
            .into()),
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
                span: $op_token.span,
            }
            .into()),
        }
    };
}
use bin_comparison_operator;

macro_rules! def_native {
    ($globals:ident . $name:ident / $arity:expr  , $fn:item) => {
        $fn
        $globals.define(
            LoxIdent { name: stringify!($name).into(), span: Span::new(0, 0) },
            LoxValue::Function(Rc::new(NativeFunction { fn_ptr: $name, arity: $arity })),
        );
    };
}
use def_native;
