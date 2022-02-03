use std::fmt::{self, Debug};

use crate::value::Value;

/// Represents a single bytecode instruction.
pub enum Ins {
    /// Return instruction. (TODO)
    Return,

    /// Constant value.
    Constant(Value),
}

impl Debug for Ins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const PAD: usize = 15;
        use Ins::*;

        match self {
            Return => f.write_str("OP_RETURN"),
            Constant(value) => write!(f, "{name:PAD$} {value:?}", name = "OP_CONSTANT"),
        }
    }
}
