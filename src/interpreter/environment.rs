use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    data::{LoxIdent, LoxValue},
    interpreter::error::RuntimeError,
};

#[derive(Debug, Default)]
struct EnvironmentInner {
    enclosing: Option<Environment>,
    locals: HashMap<String, LoxValue>,
}

#[derive(Debug, Clone, Default)]
pub struct Environment {
    inner: Rc<RefCell<EnvironmentInner>>,
}

impl Environment {
    /// Creates a new `Environment` with one scope (i.e. the global scope).
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `Environment` enclosing the given `Environment`.
    pub fn new_enclosed(enclosing: &Environment) -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvironmentInner {
                enclosing: Some(enclosing.clone()),
                locals: HashMap::new(),
            })),
        }
    }

    /// Defines a variable in the innermost scope.
    pub fn define(&mut self, ident: LoxIdent, value: LoxValue) {
        self.inner.borrow_mut().locals.insert(ident.name, value);
    }

    /// Assigns a variable.
    pub fn assign(&mut self, ident: &LoxIdent, value: LoxValue) -> Result<LoxValue, RuntimeError> {
        let mut inner = self.inner.borrow_mut();
        match inner.locals.get_mut(&ident.name) {
            Some(var) => {
                *var = value.clone();
                Ok(value)
            }
            None => match &mut inner.enclosing {
                Some(enclosing) => enclosing.assign(ident, value),
                None => Err(RuntimeError::UndefinedVariable {
                    ident: ident.clone(),
                }),
            },
        }
    }

    /// Reads a variable in a distant scope.
    pub fn assign_at(&mut self, distance: usize, ident: &LoxIdent, value: LoxValue) -> LoxValue {
        // This should never panic due to the semantic verifications that the resolver performs.
        *self
            .ancestor(distance)
            .inner
            .borrow_mut()
            .locals
            .get_mut(&ident.name)
            .unwrap() = value.clone();
        value
    }

    /// Reads a variable.
    pub fn read(&self, ident: &LoxIdent) -> Result<LoxValue, RuntimeError> {
        let inner = self.inner.borrow();
        match inner.locals.get(&ident.name) {
            Some(var) => Ok(var.clone()),
            None => match &inner.enclosing {
                Some(enclosing) => enclosing.read(ident),
                None => Err(RuntimeError::UndefinedVariable {
                    ident: ident.clone(),
                }),
            },
        }
    }

    /// Reads a variable in a distant scope.
    pub fn read_at(&self, distance: usize, ident: impl AsRef<str>) -> LoxValue {
        // This should never panic due to the semantic verifications that the resolver performs.
        self.ancestor(distance)
            .inner
            .borrow()
            .locals
            .get(ident.as_ref())
            .unwrap()
            .clone()
    }

    fn ancestor(&self, distance: usize) -> Environment {
        let mut curr = self.clone();
        for _ in 0..distance {
            let maybe_enclosing = curr.inner.borrow().enclosing.clone();
            curr = maybe_enclosing.unwrap();
        }
        curr
    }
}
