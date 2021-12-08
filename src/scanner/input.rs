use std::{iter::Peekable, str::CharIndices};

use crate::span::Span;

pub type SpannedChar = (Span, char);

pub struct Input<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
}

impl<'s> Input<'s> {
    /// Creates a new input iterator.
    pub fn new(source: &'s str) -> Input<'s> {
        Input {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    /// Peeks into the next `SpannedChar` in the iterator stream without consuming the current one.
    pub fn peek(&mut self) -> Option<SpannedChar> {
        self.chars.peek().copied().map(to_spanned)
    }

    /// Returns the source string.
    #[inline]
    pub fn source(&self) -> &'s str {
        self.source
    }

    /// Returns a slice of the source string over the given span bounds.
    #[inline]
    pub fn spanned(&self, span: Span) -> &'s str {
        &self.source[span.lo..span.hi]
    }
}

impl Iterator for Input<'_> {
    type Item = SpannedChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(to_spanned)
    }
}

/// Internal helper. Returns a spanned character.
fn to_spanned((i, c): (usize, char)) -> (Span, char) {
    (Span::new(i, i + c.len_utf8()), c)
}
