use anyhow::{bail, Result};

use crate::{
    diagnostics::Diagnostics,
    scanner::identifier::{is_valid_identifier_start, is_valid_identifier_tail, LOX_KEYWORDS},
    token::{Token, TokenKind},
};

mod identifier;

pub struct Scanner {
    chars: Vec<char>,
    tokens: Vec<Token>,
    diagnostics: Diagnostics,
    line: usize,
    current: usize,
    lexme_start: usize,
}

// The actual scanner implementation.
impl Scanner {
    /// Scans the source input string.
    pub fn scan_tokens(mut self) -> (Vec<Token>, Diagnostics) {
        while {
            self.lexme_start = self.current;
            !self.is_at_end()
        } {
            match self.scan_token_kind() {
                Ok(kind) => self.add_token(kind),
                Err(err) => self.diagnostics.diagnose(self.line, err.to_string()),
            }
        }
        self.add_token(TokenKind::Eof);
        (self.tokens, self.diagnostics)
    }

    /// Tries to scan the current character.
    pub fn scan_token_kind(&mut self) -> Result<TokenKind> {
        use TokenKind::*;
        let kind = match self.advance() {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ';' => Semicolon,
            ',' => Comma,
            '.' => Dot,
            '!' => self.take_select('=', BangEqual, Bang),
            '=' => self.take_select('=', EqualEqual, Equal),
            '>' => self.take_select('=', GreaterEqual, Greater),
            '<' => self.take_select('=', LessEqual, Less),
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '"' => self.string()?,
            '/' => self.comment_or_slash(),
            c if c.is_ascii_digit() => self.number()?,
            c if c.is_ascii_whitespace() => self.new_line_or_whitespace(c),
            c if is_valid_identifier_start(c) => self.identifier_or_keyword(),
            unexpected => bail!("Unexpected character `{}`.", unexpected),
        };
        Ok(kind)
    }

    // Tries to scan a string.
    fn string(&mut self) -> Result<TokenKind> {
        while self.current() != '"' && !self.is_at_end() {
            if self.current() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            bail!("Unterminated string");
        }
        self.advance(); // The closing `"`
        Ok(TokenKind::String(self.lexme(1, -1)))
    }

    // Scans a comment or a slash.
    fn comment_or_slash(&mut self) -> TokenKind {
        if self.take('/') {
            while self.current() != '\n' && !self.is_at_end() {
                self.advance();
            }
            TokenKind::Comment(self.lexme(2, 0))
        } else {
            TokenKind::Slash
        }
    }

    // Tries to scan a number.
    fn number(&mut self) -> Result<TokenKind> {
        while self.current().is_ascii_digit() {
            self.advance();
        }
        if self.current() == '.' && self.peek(1).is_ascii_digit() {
            self.advance(); // The `.` separator
            while self.current().is_ascii_digit() {
                self.advance();
            }
        }
        Ok(TokenKind::Number(self.lexme(0, 0).parse()?))
    }

    /// Scans a newline or a whitespace.
    fn new_line_or_whitespace(&mut self, c: char) -> TokenKind {
        if c == '\n' {
            self.line += 1;
            TokenKind::NewLine
        } else {
            TokenKind::Whitespace
        }
    }

    /// Scans a keyword or an identifier.
    fn identifier_or_keyword(&mut self) -> TokenKind {
        while is_valid_identifier_tail(self.current()) {
            self.advance();
        }
        let name = self.lexme(0, 0);
        match LOX_KEYWORDS.get(name.as_str()) {
            // Since keyword token kinds have no internal data, the following clone is cheap.
            Some(keyword_kind) => keyword_kind.clone(),
            None => TokenKind::Identifier(name),
        }
    }
}

// The scanner helper methods.
impl Scanner {
    /// Creates a new scanner.
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            tokens: Vec::new(),
            diagnostics: Diagnostics::new(),
            line: 1,
            current: 0,
            lexme_start: 0,
        }
    }

    /// Indexes the `n`th character.
    #[inline]
    fn nth(&self, n: usize) -> char {
        self.chars.get(n).copied().unwrap_or('\0')
    }

    /// Peeks a character in a given offset from the `current` cursor.
    #[inline]
    fn peek(&self, offset: usize) -> char {
        self.nth(self.current + offset)
    }

    /// Peeks into the current character (not yet consumed).
    #[inline]
    fn current(&self) -> char {
        self.peek(0)
    }

    /// Returns the current character and advances the `current` cursor.
    #[inline]
    fn advance(&mut self) -> char {
        self.current += 1;
        self.nth(self.current - 1)
    }

    /// Checks if the current character matches the given one. In such case advances and returns
    /// true. Otherwise returns false.
    #[inline]
    fn take(&mut self, expected: char) -> bool {
        if self.current() != expected {
            return false;
        }
        self.advance();
        true
    }

    /// Checks if the current character matches the given one. In such case, advances and returns
    /// `a`, otherwise returns `b`.
    #[inline]
    fn take_select<T>(&mut self, expected: char, a: T, b: T) -> T {
        match self.take(expected) {
            true => a,
            false => b,
        }
    }

    /// Checks if the scanner has finished.
    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    /// Returns the current lexme string.
    #[inline]
    fn lexme(&self, lower_bound_offset: isize, higher_bound_offset: isize) -> String {
        let lo = self.lexme_start as isize + lower_bound_offset;
        let hi = self.current as isize + higher_bound_offset;
        self.chars[lo as _..hi as _].iter().collect()
    }

    /// Creates a new token.
    #[inline]
    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            lexme: self.chars[self.lexme_start..self.current].iter().collect(),
            line: self.line,
        });
    }
}
