use std::str::Chars;

use crate::common::{Span, Token, TokenKind};

pub struct Scanner<'s> {
    // Input
    source: &'s str,
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
            '\0' => Eof,

            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ';' => Semicolon,
            ',' => Comma,
            '.' => Dot,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,

            '=' => self.next_and('=', EqualEqual, Equal),
            '!' => self.next_and('=', BangEqual, Bang),
            '<' => self.next_and('=', LessEqual, Less),
            '>' => self.next_and('=', GreaterEqual, Greater),

            // Strings
            '"' => {
                while self.peek_first() != '"' && !self.is_at_end() {
                    self.bump();
                }
                if self.is_at_end() {
                    return Error("Unterminated string".into());
                }
                self.bump(); // The closing `"`
                String(self.slice(1, 1).into())
            }

            // Numbers
            c if c.is_ascii_digit() => {
                while self.peek_first().is_ascii_digit() {
                    self.bump();
                }
                if self.peek_first() == '.' && self.peek_second().is_ascii_digit() {
                    self.bump(); // The `.`
                    while self.peek_first().is_ascii_digit() {
                        self.bump();
                    }
                }
                match self.slice(0, 0).parse() {
                    Ok(number) => Number(number),
                    Err(_) => Error("Could not parse number literal value".into()),
                }
            }

            // Keywords and identifiers
            c if is_valid_identifier_start(c) => {
                while is_valid_identifier_tail(self.peek_first()) {
                    self.bump();
                }
                let ident_str = self.slice(0, 0);
                match KEYWORDS_MAP.get(ident_str) {
                    Some(keyword_kind) => keyword_kind.clone(),
                    None => Identifier(ident_str.into()),
                }
            }

            unexpected => Error(format!("Unexpected token `{unexpected}`.")),
        }
    }

    /// Returns the next token in the source string.
    fn scan_token(&mut self) -> Token {
        // Handle ignored sequences (such as whitespaces and comments) so `scan_kind` does not
        // have to deal with them.
        self.skip_whitespace_and_comment();

        // Resets the current lexme start bound (the end bound is automatically handled by `bump`).
        self.lexme_start = self.lexme_end;

        let kind = self.scan_kind();
        let span = Span::new(self.lexme_start, self.lexme_end);
        Token { kind, span }
    }

    /// Advances *until* the next token shouldn't be ignored by the scanner.
    fn skip_whitespace_and_comment(&mut self) {
        loop {
            match self.peek_first() {
                '/' => {
                    if self.peek_second() == '/' {
                        while self.peek_first() != '\n' && !self.is_at_end() {
                            self.bump();
                        }
                    } else {
                        // The first slash must not be consumed if there is no second slash that
                        // indicates the start of a comment token (which is in fact ignored).
                        //
                        // In such case, this function is returned since the first slash must be
                        // accounted for by the "main" scanner implementation, in `scan_kind`.
                        return;
                    }
                }

                c if c.is_ascii_whitespace() => {
                    self.bump();
                }

                _ => return,
            }
        }
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
    fn peek_first(&self) -> char {
        self.source_iter.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Returns the second next character in the source string without advancing.
    fn peek_second(&self) -> char {
        let mut iter = self.source_iter.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    /// Returns `a` and advances the scanner if the first next character is equal to the expected
    /// one. Otherwise returns `b` without advancing.
    fn next_and<T>(&mut self, expected: char, a: T, b: T) -> T {
        if self.peek_first() == expected {
            self.bump();
            a
        } else {
            b
        }
    }

    /// Returns a slice over the current lexme bounds. The caller may modify the current bounds:
    ///   - A `left_modifier` will be *added* to the current *start* bound.
    ///   - A `right_modifier` will be *subtracted* from the current *end* bound.
    ///
    /// Such new computed bound must hold that `new_start_bound <= new_end_bound`.
    ///
    /// While performing the index operation, panics if the computed bound is invalid.
    fn slice(&self, left_modifier: usize, right_modifier: usize) -> &str {
        let left = self.lexme_start + left_modifier;
        let right = self.lexme_end - right_modifier;
        debug_assert!(left <= left, "Invalid computed bounds");
        &self.source[left..right]
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
            source,
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

const EOF_CHAR: char = '\0';

static KEYWORDS_MAP: phf::Map<&'static str, TokenKind> = phf::phf_map! {
    "nil"    => TokenKind::Nil,
    "true"   => TokenKind::True,
    "false"  => TokenKind::False,
    "this"   => TokenKind::This,
    "super"  => TokenKind::Super,
    "class"  => TokenKind::Class,
    "and"    => TokenKind::And,
    "or"     => TokenKind::Or,
    "if"     => TokenKind::If,
    "else"   => TokenKind::Else,
    "return" => TokenKind::Return,
    "fun"    => TokenKind::Fun,
    "for"    => TokenKind::For,
    "while"  => TokenKind::While,
    "var"    => TokenKind::Var,
    "print"  => TokenKind::Print,
    "typeof" => TokenKind::Typeof,
    "show"   => TokenKind::Show,
};

/// Checks if the given char is valid as an identifier's start character.
fn is_valid_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Checks if the given char can belong to an identifier's tail.
fn is_valid_identifier_tail(c: char) -> bool {
    c.is_ascii_digit() || is_valid_identifier_start(c)
}
