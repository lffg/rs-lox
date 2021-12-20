use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::span::Span;

#[derive(Debug)]
pub enum RuntimeError {
    UnsupportedType {
        message: String,
        operation_span: Span,
    },

    ZeroDivision {
        operation_span: Span,
    },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        // Note that a new line should NOT be included at the end. As such, while `writeln!` may be
        // called, the last call must always be an `write!`.
        match self {
            UnsupportedType {
                message,
                operation_span,
            } => {
                writeln!(f, "{}", message)?;
                write!(f, "    At position {}", operation_span)?;
                Ok(())
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
