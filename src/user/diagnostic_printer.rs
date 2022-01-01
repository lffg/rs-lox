use std::io::Write;

use crate::span::Span;
use ansi_term::Color::Red;

pub fn print_span_window(writer: &mut dyn Write, str: &str, span: Span) {
    let spanned = SpannedStr::new(str, span);

    // TODO: Handle multi lines
    let (before, span, after) = spanned.line_sections();
    writeln!(
        writer,
        "{:>5} | {}{}{}",
        spanned.line_number(),
        before,
        Red.paint(span),
        after
    )
    .unwrap();
}

struct SpannedStr<'s> {
    str: &'s str,
    span: Span,
}

impl<'s> SpannedStr<'s> {
    pub fn new(str: &'s str, span: Span) -> SpannedStr<'s> {
        Self { str, span }
    }

    pub fn line_number(&self) -> usize {
        &self.str[..self.span.lo].matches('\n').count() + 1
    }

    pub fn line_bounds(&self) -> (usize, usize) {
        let Span { lo, hi } = self.span;
        let len = self.str.len();

        let before = &self.str[..lo];
        let start = before.rfind('\n').map(|pos| pos + 1).unwrap_or(0);

        let after = &self.str[hi..];
        let end = after.find('\n').map(|pos| pos + hi + 1).unwrap_or(len);

        (start, end)
    }

    pub fn line_sections(&self) -> (&str, &str, &str) {
        let (l_start, l_end) = self.line_bounds();

        let before = &self.str[l_start..self.span.lo];
        let span = &self.str[self.span.range()];
        let after = &self.str[self.span.hi..l_end];
        (before, span, after)
    }
}
