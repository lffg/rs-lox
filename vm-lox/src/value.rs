use std::fmt::{self, Display};

/// Represents a Lox value.
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(number) => {
                if number.floor() == *number {
                    write!(f, "{:.0}", number)
                } else {
                    write!(f, "{}", number)
                }
            }
        }
    }
}
