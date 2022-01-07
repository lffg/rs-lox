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

impl LoxIdent {
    pub fn new(span: Span, name: impl Into<String>) -> Self {
        LoxIdent {
            id: AstId::new(),
            name: name.into(),
            span,
        }
    }
}

impl From<Token> for LoxIdent {
    fn from(Token { kind, span }: Token) -> Self {
        match kind {
            TokenKind::Identifier(name) => LoxIdent::new(span, name),
            unexpected => unreachable!(
                "Invalid `Token` ({:?}) to `LoxIdent` conversion.",
                unexpected
            ),
        }
    }
}

impl AsRef<str> for LoxIdent {
    fn as_ref(&self) -> &str {
        &self.name
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

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub decl: Rc<FunDecl>,
    pub closure: Environment,
    pub is_class_init: bool,
}

impl LoxFunction {
    pub fn bind(&self, instance: &Rc<LoxInstance>) -> Rc<Self> {
        let mut env = Environment::new_enclosed(&self.closure);
        env.define(
            LoxIdent::new(Span::new(0, 0), "this"),
            LoxValue::Object(instance.clone()),
        );
        Rc::new(LoxFunction {
            decl: self.decl.clone(),
            closure: env,
            is_class_init: self.is_class_init,
        })
    }
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
        let real_returned_value = match interpreter.eval_block(&self.decl.body, env) {
            Ok(()) => LoxValue::Nil,
            Err(ControlFlow::Return(value)) => value,
            Err(other) => return Err(other),
        };
        // If the function being currently executed happens to be the initializer (i.e. "init") of
        // some class, the returned value should be simply ignored, since it always returns the
        // instance's `this` value implicitly by this implementation (it's a Lox design choice).
        //
        // Note that if an error arises from the initializer it is not ignored.
        if self.is_class_init {
            Ok(self.closure.read_at(0, "this"))
        } else {
            Ok(real_returned_value)
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
    pub methods: HashMap<String, Rc<LoxFunction>>,
}

impl LoxClass {
    pub fn get_method(&self, ident: impl AsRef<str>) -> Option<Rc<LoxFunction>> {
        self.methods.get(ident.as_ref()).cloned()
    }
}

// Class instantiation.
impl LoxCallable for LoxClass {
    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        args: &[LoxValue],
    ) -> CFResult<LoxValue> {
        let instance = Rc::new(LoxInstance {
            constructor: self,
            properties: RefCell::new(HashMap::new()),
        });
        // Run the class' initializer if it's defined.
        if let Some(init) = instance.get_bound_method("init") {
            init.call(interpreter, args)?;
        }
        Ok(LoxValue::Object(instance))
    }

    fn arity(&self) -> usize {
        match self.get_method("init") {
            Some(function) => function.arity(),
            None => 0,
        }
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
    pub fn get(self: &Rc<Self>, ident: &LoxIdent) -> Result<LoxValue, RuntimeError> {
        if let Some(value) = self.properties.borrow().get(&ident.name) {
            return Ok(value.clone());
        }

        if let Some(method) = self.get_bound_method(ident) {
            return Ok(LoxValue::Function(method));
        }

        Err(RuntimeError::UndefinedProperty {
            ident: ident.clone(),
        })
    }

    pub fn set(&self, ident: &LoxIdent, value: LoxValue) {
        self.properties
            .borrow_mut()
            .insert(ident.name.clone(), value);
    }

    pub fn get_bound_method(self: &Rc<Self>, ident: impl AsRef<str>) -> Option<Rc<LoxFunction>> {
        self.constructor
            .get_method(ident)
            .map(|unbound| unbound.bind(self))
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<object {} {{", self.constructor.name)?;
        for (i, (key, val)) in self.properties.borrow().iter().enumerate() {
            if i == 0 {
                writeln!(f)?;
            }
            writeln!(f, "  {}: {:?}", key, val)?;
        }
        write!(f, "}}>")?;
        Ok(())
    }
}
