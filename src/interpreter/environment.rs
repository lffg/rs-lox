#![allow(dead_code)]

use std::collections::HashMap;

use crate::value::LoxValue;

use super::{IResult, RuntimeError};

#[derive(Debug)]
pub struct Environment {
    // This implementation is currently suboptimal due to `LoxValue` cloning and the use of owned
    // Strings as keys. Most of the current issues would be solved once string interning is
    // implemented and used to store lox's string values and identifiers.
    scopes: Vec<HashMap<String, LoxValue>>,
}

impl Environment {
    /// Creates a new `Environment` with one scope (i.e. the global scope).
    pub fn new() -> Self {
        Self {
            scopes: Vec::from([HashMap::new()]),
        }
    }

    /// Appends a new empty scope to the `scopes` stack.
    pub fn add_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Removes the innermost scope from the `scopes` stack.
    ///
    /// # Panics
    ///
    /// This will panic if there is only one scope in the stack (which must not be dropped as it is
    /// the global scope).
    pub fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1, "Cannot drop the global scope.");
        self.scopes.pop();
    }

    /// Defines a variable in the innermost scope.
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    /// Assigns a variable.
    pub fn assign(&mut self, name: &str, value: LoxValue) -> IResult<LoxValue> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value_ref) = scope.get_mut(name) {
                *value_ref = value.clone();
                return Ok(value);
            }
        }
        Err(RuntimeError::UndefinedVariable { name: name.into() })
    }

    /// Reads a variable.
    pub fn read(&self, name: &str) -> IResult<LoxValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(RuntimeError::UndefinedVariable { name: name.into() })
    }
}
