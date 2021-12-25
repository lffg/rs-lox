use std::collections::HashMap;

use crate::value::LoxValue;

use super::{IResult, RuntimeError};

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn read(&self, name: &str) -> IResult<LoxValue> {
        self.values
            .get(name)
            .cloned() // TODO: Remove this, allow for `copied`.
            .ok_or_else(|| RuntimeError::UndefinedVariable { name: name.into() })
    }
}
