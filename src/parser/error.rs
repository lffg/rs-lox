use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::{
    span::Span,
    token::{Token, TokenKind},
};

pub type PResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        message: String,
        offending: Token,
        expected: Option<TokenKind>,
    },

    ScannerError {
        message: String,
        offending_span: Span,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        // Note that a new line should NOT be included at the end. As such, while `writeln!` may be
        // called, the last call must always be an `write!`.
        match self {
            UnexpectedToken {
                message,
                offending,
                expected,
            } => {
                writeln!(f, "{}", message)?;
                write!(f, "    Unexpected token {}", offending)?;
                if let Some(expected) = expected {
                    write!(f, "\n    Expected token {}", expected)?;
                }
                Ok(())
            }

            ScannerError {
                message,
                offending_span,
            } => {
                writeln!(f, "{}", message)?;
                write!(f, "    At {}", offending_span)?;
                Ok(())
            }
        }
    }
}

impl Error for ParseError {}
