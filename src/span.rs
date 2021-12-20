use std::{
    cmp::{max, min},
    fmt::{self, Display},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
/// Represents a string fragment. The bounds are over its byte representation.
pub struct Span {
    /// Lower bound.
    pub lo: usize,

    /// Higher bound.
    pub hi: usize,
}

impl Span {
    /// Creates a new span.
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

    /// Modifies the given span. Panics if new bounds are invalid.
    pub fn updated(&self, lo: isize, hi: isize) -> Span {
        let lo = self.lo as isize + lo;
        let hi = self.hi as isize + hi;
        assert!(lo >= 0, "New lower bound can't be negative.");
        assert!(lo <= hi, "Lower bound can not pass the higher.");
        Span::new(lo as _, hi as _)
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
