use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::{
    parser::scanner::error::ScanError,
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Error {
        message: String,
        span: Span,
    },

    ScanError {
        error: ScanError,
        span: Span,
    },

    UnexpectedToken {
        message: String,
        offending: Token,
        expected: Option<TokenKind>,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        match self {
            Error { message, span } => {
                writeln!(f, "{}", message)?;
                write!(f, "    At position {}", span)?;
                Ok(())
            }

            ScanError { error, span } => {
                writeln!(f, "{}", error)?;
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
        }
    }
}

impl Error for ParseError {}

impl ParseError {
    /// Checks if the error allows REPL continuation (aka. "..." prompt).
    pub fn allows_continuation(&self) -> bool {
        use ParseError::*;
        match self {
            UnexpectedToken { offending, .. } if offending.kind == TokenKind::Eof => true,
            ScanError { error, .. } if error.allows_continuation() => true,
            _ => false,
        }
    }
}
