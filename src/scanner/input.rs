use std::str::CharIndices;

use peekmore::{PeekMore, PeekMoreIterator};

use crate::span::Span;

pub type SpannedChar = (Span, char);

pub struct Input<'s> {
    source: &'s str,
    chars: PeekMoreIterator<CharIndices<'s>>,
    current: SpannedChar,
}

impl<'s> Input<'s> {
    /// Creates a new input iterator.
    pub fn new(source: &'s str) -> Input<'s> {
        let mut chars = source.char_indices().peekmore();
        let current = chars.next().map(to_spanned).unwrap_or_default();
        Input {
            source,
            chars,
            current,
        }
    }

    /// Returns the current `SpannedChar`.
    #[inline]
    pub fn current(&self) -> SpannedChar {
        self.current
    }

    /// Advances the input. Returns the new current char.
    pub fn advance(&mut self) -> SpannedChar {
        self.current = unwrap_spanned(self.chars.next().map(to_spanned), self.source.len());
        self.current
    }

    /// Peeks into the next `SpannedChar` in the iterator stream without consuming the current one.
    pub fn peek(&mut self) -> SpannedChar {
        unwrap_spanned(
            self.chars.peek().copied().map(to_spanned),
            self.source.len(),
        )
    }

    /// Peeks into the `n`th char in the iterator.
    #[inline]
    pub fn peek_nth_char(&mut self, n: usize) -> char {
        self.chars.peek_nth(n).map(|(_, c)| *c).unwrap_or('\0')
    }

    /// Checks if the input is finished.
    #[inline]
    pub fn finished(&self) -> bool {
        self.current.1 == '\0'
    }

    /// Returns the source string.
    #[inline]
    pub fn source(&self) -> &'s str {
        self.source
    }

    /// Returns a slice of the source string over the given span bounds.
    #[inline]
    pub fn spanned(&self, span: Span) -> &'s str {
        &self.source()[span.lo..span.hi]
    }
}

/// Returns a spanned character.
#[inline]
fn to_spanned((i, c): (usize, char)) -> (Span, char) {
    (Span::new(i, i + c.len_utf8()), c)
}

/// Unwraps the given `SpannedChar` with custom span.
#[inline]
fn unwrap_spanned(span: Option<SpannedChar>, len: usize) -> SpannedChar {
    span.unwrap_or_else(|| (Span::new(len, len), '\0'))
}
