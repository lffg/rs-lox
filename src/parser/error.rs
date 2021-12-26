use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::{
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub enum ParseError {
    // The most generic error kind.
    Error {
        message: String,
        span: Span,
    },

    UnexpectedToken {
        message: String,
        offending: Token,
        expected: Option<TokenKind>,
    },

    ScannerError {
        message: String,
        span: Span,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        // Note that a new line should NOT be included at the end. As such, while `writeln!` may be
        // called, the last call must always be an `write!`.
        match self {
            Error { message, span } => {
                writeln!(f, "{}", message)?;
                write!(f, "    At position {}", span)?;
                Ok(())
            }

            UnexpectedToken {
                message,
                offending,
                expected,
            } => {
                writeln!(f, "{}", message)?;
                write!(
                    f,
                    "    Unexpected token `{}` at position {}",
                    offending, offending.span
                )?;
                if let Some(expected) = expected {
                    write!(f, "\n    Expected token `{}`", expected)?;
                }
                Ok(())
            }

            ScannerError { message, span } => {
                writeln!(f, "{}", message)?;
                write!(f, "    At position {}", span)?;
                Ok(())
            }
        }
    }
}

impl Error for ParseError {}
