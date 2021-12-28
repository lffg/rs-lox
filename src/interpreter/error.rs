use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    UnsupportedType {
        message: String,
        operation_span: Span,
    },

    UndefinedVariable {
        name: String,
    },

    ZeroDivision {
        operation_span: Span,
    },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            UnsupportedType {
                message,
                operation_span,
            } => {
                writeln!(f, "{}", message)?;
                write!(f, "    At position {}", operation_span)?;
                Ok(())
            }

            UndefinedVariable { name } => {
                write!(f, "Undefined variable `{}`", name)
            }

            ZeroDivision { operation_span } => {
                writeln!(f, "Can not divide by zero")?;
                write!(f, "    At position {}", operation_span)?;
                Ok(())
            }
        }
    }
}

impl Error for RuntimeError {}
