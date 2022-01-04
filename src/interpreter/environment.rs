use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    data::{LoxIdent, LoxValue},
    interpreter::{error::RuntimeError, CFResult},
};

#[derive(Debug, Default)]
struct EnvironmentInner {
    enclosing: Option<Rc<RefCell<EnvironmentInner>>>,
    locals: HashMap<String, LoxValue>,
}

#[derive(Debug, Default)]
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
                enclosing: Some(Rc::clone(&enclosing.inner)),
                locals: HashMap::new(),
            })),
        }
    }

    /// Defines a variable in the innermost scope.
    pub fn define(&mut self, ident: LoxIdent, value: LoxValue) {
        self.inner.borrow_mut().locals.insert(ident.name, value);
    }

    /// Assigns a variable. Returns `None` in case of undefined variable error.
    pub fn assign(&mut self, ident: &LoxIdent, value: LoxValue) -> CFResult<LoxValue> {
        let mut maybe_inner = Some(self.inner.clone()); // this clone is cheap (Rc)
        while let Some(inner) = maybe_inner {
            let mut inner = inner.borrow_mut();
            if let Some(value_ref) = inner.locals.get_mut(&ident.name) {
                *value_ref = value.clone();
                return Ok(value);
            }
            maybe_inner = inner.enclosing.clone(); // this clone is cheap (Rc)
        }
        Err(RuntimeError::UndefinedVariable {
            ident: ident.clone(),
        }
        .into())
    }

    /// Reads a variable. Returns `None` in case of undefined variable error.
    pub fn read(&self, ident: &LoxIdent) -> CFResult<LoxValue> {
        let mut maybe_inner = Some(self.inner.clone()); // this clone is cheap (Rc)
        while let Some(inner) = maybe_inner {
            let inner = inner.borrow();
            if let Some(value) = inner.locals.get(&ident.name) {
                return Ok(value.clone());
            }
            maybe_inner = inner.enclosing.clone(); // this clone is cheap (Rc)
        }
        Err(RuntimeError::UndefinedVariable {
            ident: ident.clone(),
        }
        .into())
    }
}
