use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::data::LoxValue;

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

    pub fn new_enclosed(enclosing: &Environment) -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvironmentInner {
                enclosing: Some(Rc::clone(&enclosing.inner)),
                locals: HashMap::new(),
            })),
        }
    }

    /// Defines a variable in the innermost scope.
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.inner.borrow_mut().locals.insert(name, value);
    }

    /// Assigns a variable. Returns `None` in case of undefined variable error.
    #[must_use]
    pub fn assign(&mut self, name: &str, value: LoxValue) -> Option<LoxValue> {
        let mut maybe_inner = Some(self.inner.clone()); // this clone is cheap (Rc)
        while let Some(inner) = maybe_inner {
            let mut inner = inner.borrow_mut();
            if let Some(value_ref) = inner.locals.get_mut(name) {
                *value_ref = value.clone();
                return Some(value);
            }
            maybe_inner = inner.enclosing.clone(); // this clone is cheap (Rc)
        }
        None
    }

    /// Reads a variable. Returns `None` in case of undefined variable error.
    #[must_use]
    pub fn read(&self, name: &str) -> Option<LoxValue> {
        let mut maybe_inner = Some(self.inner.clone()); // this clone is cheap (Rc)
        while let Some(inner) = maybe_inner {
            let inner = inner.borrow();
            if let Some(value) = inner.locals.get(name) {
                return Some(value.clone());
            }
            maybe_inner = inner.enclosing.clone(); // this clone is cheap (Rc)
        }
        None
    }
}
