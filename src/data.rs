use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::{
    ast::stmt::Fun,
    interpreter::{environment::Environment, IResult, Interpreter},
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub enum LoxValue {
    Function(Rc<dyn LoxCallable>),
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl LoxValue {
    /// Returns the canonical type name.
    pub fn type_name(&self) -> &'static str {
        use LoxValue::*;
        match self {
            Function(_) => "function",
            Boolean(_) => "boolean",
            Number(_) => "number",
            String(_) => "string",
            Nil => "nil",
        }
    }
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxValue::*;
        match self {
            Function(_) => f.write_str("<fun>"),
            Boolean(boolean) => Display::fmt(boolean, f),
            Number(number) => {
                if number.floor() == *number {
                    write!(f, "{:.0}", number)
                } else {
                    Display::fmt(number, f)
                }
            }
            String(string) => f.write_str(string),
            Nil => f.write_str("nil"),
        }
    }
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxValue::*;
        match self {
            String(s) => write!(f, "\"{}\"", s),
            other => Display::fmt(other, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoxIdent {
    pub name: String,
    pub span: Span,
}

impl From<Token> for LoxIdent {
    fn from(Token { kind, span }: Token) -> Self {
        match kind {
            TokenKind::Identifier(name) => LoxIdent { name, span },
            unexpected => unreachable!(
                "Invalid `Token` ({:?}) to `LoxIdent` conversion.",
                unexpected
            ),
        }
    }
}

impl Display for LoxIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxValue]) -> IResult<LoxValue>;
    fn arity(&self) -> usize;
}

pub struct LoxFunction {
    pub fun_stmt: Fun,
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxValue]) -> IResult<LoxValue> {
        let mut env = Environment::new();
        for (param, value) in self.fun_stmt.params.iter().zip(args) {
            env.define(param.clone(), value.clone());
        }
        interpreter.eval_block(&self.fun_stmt.body, env)?;
        Ok(LoxValue::Nil)
    }

    fn arity(&self) -> usize {
        self.fun_stmt.params.len()
    }
}
pub struct NativeFunction {
    pub fn_ptr: fn(args: &[LoxValue]) -> IResult<LoxValue>,
    pub arity: usize,
}

impl LoxCallable for NativeFunction {
    fn call(&self, _: &mut Interpreter, args: &[LoxValue]) -> IResult<LoxValue> {
        (self.fn_ptr)(args)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}
