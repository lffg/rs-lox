#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::LoxValue;

use super::{IResult, RuntimeError};

#[derive(Debug, Default)]
pub struct Environment {
    // Since more than one environment may have the same enclosing an mutations might happen (i.e.
    // when assigning to an already defined variable), reference counting and interior mutability
    // are required.
    enclosing: Option<Rc<RefCell<Environment>>>,
    // This implementation is currently suboptimal due to `LoxValue` cloning and the use of owned
    // Strings as keys. Most of the current issues would be solved once string interning is
    // implemented and used to store lox's string values and identifiers.
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: LoxValue) -> IResult<LoxValue> {
        match self.values.get_mut(name) {
            Some(value_ref) => {
                *value_ref = value.clone();
                Ok(value)
            }
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(RuntimeError::UndefinedVariable { name: name.into() }),
            },
        }
    }

    pub fn read(&self, name: &str) -> IResult<LoxValue> {
        match self.values.get(name) {
            Some(name) => Ok(name.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().read(name),
                None => Err(RuntimeError::UndefinedVariable { name: name.into() }),
            },
        }
    }
}
