use std::{
    cmp::{max, min},
    fmt::{self, Display},
    ops::Range,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
/// Represents a string fragment.
pub struct Span {
    /// Substring start index.
    pub lo: usize,

    /// Substring end index (not inclusive).
    pub hi: usize,
}

impl Span {
    /// Creates a new span from the given bounds.
    pub fn new(lo: usize, hi: usize) -> Span {
        Span {
            lo: min(lo, hi),
            hi: max(lo, hi),
        }
    }

    /// Creates a new span encompassing `self` and `other`.
    pub fn to(&self, other: Span) -> Span {
        Span::new(min(self.lo, other.lo), max(self.hi, other.hi))
    }

    /// Returns the span `Range<usize>` representation.
    pub fn into_range(&self) -> Range<usize> {
        Range {
            start: self.lo,
            end: self.hi,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (self.hi - self.lo) <= 1 {
            write!(f, "{}", self.lo)
        } else {
            write!(f, "{}..{}", self.lo, self.hi)
        }
    }
}
