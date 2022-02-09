use std::str::Chars;

use crate::common::{Span, Token, TokenKind};

const EOF_CHAR: char = '\0';

pub struct Scanner<'s> {
    // Input
    source_iter: Chars<'s>,
    // Lexme location
    lexme_start: usize,
    lexme_end: usize,
    // Is it finished?
    done: bool,
}

// Core implementation.
impl Scanner<'_> {
    /// Returns the next kind of token in the source string.
    fn scan_kind(&mut self) -> TokenKind {
        let current = self.bump();

        use TokenKind::*;
        match current {
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '\0' => Eof,
            unexpected => Error(format!("Unexpected token `{unexpected}`.")),
        }
    }

    /// Returns the next token in the source string.
    fn scan_token(&mut self) -> Token {
        // Current `lexme_start` should be the previously produced token's `lexme_end`.
        // This in fact means the start of a new lexme "registering".
        self.lexme_start = self.lexme_end;

        // Calling `scan` must happen before computing the `text` field.
        let kind = self.scan_kind();
        let span = Span::new(self.lexme_start, self.lexme_end);
        Token { kind, span }
    }
}

// Utilities.
impl Scanner<'_> {
    /// Advances the underlying character iterator and returns the yielded character. Returns the
    /// null character ('\0') if there are no more characters to be produced.
    ///
    /// Increments the `lexme_end` field according to the returned character's UTF-8 length.
    fn bump(&mut self) -> char {
        self.source_iter
            .next()
            .map(|char| {
                self.lexme_end += char.len_utf8();
                char
            })
            .unwrap_or_else(|| {
                if self.done {
                    panic!("Scanner must not advance past the end of input");
                }
                self.done = true;
                EOF_CHAR
            })
    }

    /// Returns the first next character in the source string without advancing.
    fn first(&self) -> char {
        self.source_iter.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Returns the second next character in the source string without advancing.
    fn second(&self) -> char {
        self.source_iter.clone().nth(1).unwrap_or(EOF_CHAR)
    }

    /// Checks if the source string is finished.
    fn is_at_end(&self) -> bool {
        self.source_iter.as_str().is_empty()
    }
}

// Public API.
impl<'s> Scanner<'s> {
    /// Creates a new scanner from the given source string.
    pub fn new(source: &'s str) -> Scanner<'s> {
        Scanner {
            source_iter: source.chars(),
            lexme_start: 0,
            lexme_end: 0,
            done: false,
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        Some(self.scan_token())
    }
}
