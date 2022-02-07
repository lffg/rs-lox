use std::fmt::{self, Debug};

use crate::common::Value;

/// Represents a single bytecode instruction.
pub enum Ins {
    /// Constant value.
    Constant(Value),

    /// Negation.
    Negate,

    /// Addition.
    Add,

    /// Subtraction.
    Subtract,

    /// Multiplication.
    Multiply,

    /// Division.
    Divide,

    /// Return instruction.
    Return,
}

impl Debug for Ins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const PAD: usize = 15;
        use Ins::*;
        match self {
            Constant(value) => write!(f, "{name:PAD$} {value:?}", name = "OP_CONSTANT"),

            Negate => f.write_str("OP_NEGATE"),
            Add => f.write_str("OP_ADD"),
            Subtract => f.write_str("OP_NEGATE"),
            Multiply => f.write_str("OP_MULTIPLY"),
            Divide => f.write_str("OP_DIVIDE"),

            Return => f.write_str("OP_RETURN"),
        }
    }
}
