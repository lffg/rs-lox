use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    UnsupportedType { message: String, span: Span },

    UndefinedVariable { name: String, span: Span },

    ZeroDivision { span: Span },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            UnsupportedType { message, span } => {
                write!(f, "{}; at position {}", message, span)
            }

            UndefinedVariable { name, span } => {
                write!(f, "Undefined variable `{}`; at position {}", name, span)
            }

            ZeroDivision { span } => {
                write!(f, "Can not divide by zero; at position {}", span)
            }
        }
    }
}

impl RuntimeError {
    /// Returns the span that caused the error.
    pub fn primary_span(&self) -> Span {
        use RuntimeError::*;
        match self {
            UnsupportedType { span, .. }
            | UndefinedVariable { span, .. }
            | ZeroDivision { span } => *span,
        }
    }
}

impl Error for RuntimeError {}
