use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::interpreter::{IResult, Interpreter};

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

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxValue]) -> IResult<LoxValue>;
    fn arity(&self) -> usize;
}

pub struct NativeFunction {
    pub ptr: fn(args: &[LoxValue]) -> IResult<LoxValue>,
    pub arity: usize,
}

impl LoxCallable for NativeFunction {
    fn call(&self, _: &mut Interpreter, args: &[LoxValue]) -> IResult<LoxValue> {
        (self.ptr)(args)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}
