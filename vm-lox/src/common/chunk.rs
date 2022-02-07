use std::fmt::{self, Debug};

use crate::common::Ins;

/// Represents a chunk of bytecode. A sequence of instructions.
pub struct Chunk {
    name: String,
    pub(crate) code: Vec<Ins>,
    pub(crate) lines: Vec<u32>,
}

impl Chunk {
    /// Creates a new chunk.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            code: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Writes an instruction to the chunk's bytecode.
    pub fn write(&mut self, ins: Ins, line: u32) {
        debug_assert_eq!(
            self.code.len(),
            self.lines.len(),
            "Not parallel lengths of code and lines vectors"
        );

        self.code.push(ins);
        self.lines.push(line);
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== {} ===", self.name)?;

        let mut last_line = 0;
        for (ins, &line) in self.code.iter().zip(&self.lines) {
            if last_line != line {
                write!(f, "{line:>5}")?;
                last_line = line;
            } else {
                f.write_str("    .")?;
            }
            writeln!(f, " | {ins:?}")?;
        }

        Ok(())
    }
}
