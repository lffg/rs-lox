use crate::{chunk::Chunk, ins::Ins};

/// The virtual machine.
pub struct Vm;

impl Vm {
    pub fn new() -> Vm {
        Self
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), ()> {
        for ins in chunk.code {
            use Ins::*;
            match ins {
                Return => return Ok(()),
                Constant(value) => {
                    println!("{value:?}");
                }
            }
        }
        Ok(())
    }
}
