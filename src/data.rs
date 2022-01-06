use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::{
    ast::{stmt::FunDecl, AstId},
    interpreter::{
        control_flow::ControlFlow, environment::Environment, error::RuntimeError, CFResult,
        Interpreter,
    },
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub enum LoxValue {
    Function(Rc<dyn LoxCallable>),
    Object(Rc<LoxInstance>),
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
            Object(_) => "object",
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
            Function(fun) => Display::fmt(fun, f),
            Object(instance) => Display::fmt(instance, f),
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
    pub id: AstId,
}

impl From<Token> for LoxIdent {
    fn from(Token { kind, span }: Token) -> Self {
        match kind {
            TokenKind::Identifier(name) => {
                let id = AstId::new();
                LoxIdent { name, span, id }
            }
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

pub trait LoxCallable: Display + Debug {
    fn call(self: Rc<Self>, interpreter: &mut Interpreter, args: &[LoxValue])
        -> CFResult<LoxValue>;
    fn arity(&self) -> usize;
}

#[derive(Debug)]
pub struct LoxFunction {
    pub decl: FunDecl,
    pub closure: Environment,
}

impl LoxCallable for LoxFunction {
    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        args: &[LoxValue],
    ) -> CFResult<LoxValue> {
        let mut env = Environment::new_enclosed(&self.closure);
        for (param, value) in self.decl.params.iter().zip(args) {
            env.define(param.clone(), value.clone());
        }
        match interpreter.eval_block(&self.decl.body, env) {
            Ok(()) => Ok(LoxValue::Nil),
            Err(ControlFlow::Return(value)) => Ok(value),
            Err(other) => Err(other),
        }
    }

    fn arity(&self) -> usize {
        self.decl.params.len()
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fun {}>", self.decl.name)
    }
}

pub struct NativeFunction {
    pub name: &'static str,
    pub fn_ptr: fn(args: &[LoxValue]) -> CFResult<LoxValue>,
    pub arity: usize,
}

impl LoxCallable for NativeFunction {
    fn call(self: Rc<Self>, _: &mut Interpreter, args: &[LoxValue]) -> CFResult<LoxValue> {
        (self.fn_ptr)(args)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fun (native) {}>", self.name)
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFunction")
            .field("name", &self.name)
            .field("fn_ptr", &"fn_ptr")
            .field("arity", &self.arity)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: LoxIdent,
}

impl LoxCallable for LoxClass {
    fn call(self: Rc<Self>, _: &mut Interpreter, _: &[LoxValue]) -> CFResult<LoxValue> {
        let instance = LoxInstance {
            constructor: self,
            properties: RefCell::new(HashMap::new()),
        };
        Ok(LoxValue::Object(Rc::new(instance)))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub constructor: Rc<LoxClass>,
    properties: RefCell<HashMap<String, LoxValue>>,
}

impl LoxInstance {
    pub fn get(&self, ident: &LoxIdent) -> Result<LoxValue, RuntimeError> {
        match self.properties.borrow().get(&ident.name) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::UndefinedProperty {
                ident: ident.clone(),
            }),
        }
    }

    pub fn set(&self, ident: &LoxIdent, value: LoxValue) {
        self.properties
            .borrow_mut()
            .insert(ident.name.clone(), value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<object {} {{", self.constructor.name)?;
        for (i, (key, val)) in self.properties.borrow().iter().enumerate() {
            if i == 0 {
                writeln!(f)?;
            }
            writeln!(f, "  {}: {}", key, val)?;
        }
        write!(f, "}}>")?;
        Ok(())
    }
}
