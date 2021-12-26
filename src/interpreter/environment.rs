use std::collections::HashMap;

use crate::value::LoxValue;

use super::{IResult, RuntimeError};

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
}

// This implementation is currently suboptimal due to `LoxValue` cloning and the use of owned
// Strings as keys. Most of the current issues would be solved once string interning is implemented
// and used to store lox string values and lox identifiers.

impl Environment {
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: LoxValue) -> IResult<LoxValue> {
        match self.values.get_mut(name) {
            Some(value_ref) => {
                *value_ref = value.clone();
                Ok(value)
            }
            None => Err(RuntimeError::UndefinedVariable { name: name.into() }),
        }
    }

    pub fn read(&self, name: &str) -> IResult<LoxValue> {
        match self.values.get(name) {
            Some(name) => Ok(name.clone()),
            None => Err(RuntimeError::UndefinedVariable { name: name.into() }),
        }
    }
}
