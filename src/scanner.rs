use crate::{
    scanner::identifier::{is_valid_identifier_start, is_valid_identifier_tail, LOX_KEYWORDS},
    span::Span,
    token::{Token, TokenKind},
};

mod identifier;

pub struct Scanner<'src> {
    src: &'src str,
    chars: Vec<(usize, char)>, // Start byte index and char
    cursor: usize,
    lexme_span_start: usize,
    emitted_eof: bool,
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.emitted_eof {
            return None;
        }
        // Ensures the next token starts with a new span.
        self.lexme_span_start = self.peek(0).0;
        let kind = self.scan_token_kind();
        if kind == TokenKind::Eof {
            self.emitted_eof = true;
        }
        Some(Token {
            kind,
            span: self.lexme_span(),
        })
    }
}

// The scanner implementation.
impl Scanner<'_> {
    /// Tries to scan the current character.
    pub fn scan_token_kind(&mut self) -> TokenKind {
        use TokenKind::*;
        match self.advance() {
            '\0' => Eof,
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
            '"' => self.string(),
            '/' => self.comment_or_slash(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_whitespace() => self.new_line_or_whitespace(c),
            c if is_valid_identifier_start(c) => self.identifier_or_keyword(),
            unexpected => Error(format!("Unexpected character `{}`.", unexpected)),
        }
    }

    /// Tries to scan a string.
    fn string(&mut self) -> TokenKind {
        while self.current() != '"' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            return TokenKind::Error("Unterminated string.".into());
        }
        self.advance(); // The closing `"`
        TokenKind::String(self.lexme(1, -1).into())
    }

    /// Scans a comment or a slash.
    fn comment_or_slash(&mut self) -> TokenKind {
        if self.take('/') {
            while self.current() != '\n' && !self.is_at_end() {
                self.advance();
            }
            TokenKind::Comment(self.lexme(2, 0).into())
        } else {
            TokenKind::Slash
        }
    }

    /// Tries to scan a number.
    fn number(&mut self) -> TokenKind {
        while self.current().is_ascii_digit() {
            self.advance();
        }
        if self.current() == '.' && self.peek(1).1.is_ascii_digit() {
            self.advance(); // The `.` separator
            while self.current().is_ascii_digit() {
                self.advance();
            }
        }
        match self.lexme(0, 0).parse() {
            Ok(parsed) => TokenKind::Number(parsed),
            Err(_) => TokenKind::Error("Unparseable number literal.".into()),
        }
    }

    /// Scans a newline or a whitespace.
    fn new_line_or_whitespace(&mut self, c: char) -> TokenKind {
        match c {
            '\n' => TokenKind::NewLine,
            _ => TokenKind::Whitespace,
        }
    }

    /// Scans a keyword or an identifier.
    fn identifier_or_keyword(&mut self) -> TokenKind {
        while is_valid_identifier_tail(self.current()) {
            self.advance();
        }
        let name = self.lexme(0, 0);
        match LOX_KEYWORDS.get(name) {
            // Since keyword token kinds have no internal data, the following clone is cheap.
            Some(keyword_kind) => keyword_kind.clone(),
            None => TokenKind::Identifier(name.into()),
        }
    }
}

// The scanner helper methods.
impl<'src> Scanner<'src> {
    /// Creates a new scanner.
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            chars: src.char_indices().collect(),
            cursor: 0,
            lexme_span_start: 0,
            emitted_eof: false,
        }
    }

    /// Peeks a character tuple with the given offset from the cursor.
    #[inline]
    fn peek(&self, offset: isize) -> (usize, char) {
        self.chars
            .get((self.cursor as isize + offset) as usize)
            .copied()
            .unwrap_or((self.src.len(), '\0'))
    }

    /// Peeks into the current character (not yet consumed).
    #[inline]
    fn current(&self) -> char {
        self.peek(0).1
    }

    /// Returns the current character and advances the `current` cursor.
    #[inline]
    fn advance(&mut self) -> char {
        self.cursor += 1;
        self.peek(-1).1
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

    /// Returns the current lexme span.
    #[inline]
    fn lexme_span(&self) -> Span {
        Span::new(self.lexme_span_start, self.peek(0).0)
    }

    /// Returns a lexme slice.
    #[inline]
    fn lexme(&self, lo: isize, hi: isize) -> &'src str {
        let span = self.lexme_span().updated(lo, hi);
        &self.src[span.lo..span.hi]
    }

    /// Checks if the scanner has finished.
    #[inline]
    fn is_at_end(&self) -> bool {
        self.cursor >= self.chars.len()
    }
}
