use std::fmt::{self, Debug};

use crate::value::Value;

/// Represents a single bytecode instruction.
pub enum Ins {
    /// Constant value.
    Constant(Value),

    /// Takes one operand, the value to negate.
    /// Produces a single result value.
    Negate,

    /// Return instruction. (TODO)
    Return,
}

impl Debug for Ins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const PAD: usize = 15;
        use Ins::*;
        match self {
            Constant(value) => write!(f, "{name:PAD$} {value:?}", name = "OP_CONSTANT"),

            Negate => f.write_str("OP_NEGATE"),

            Return => f.write_str("OP_RETURN"),
        }
    }
}
