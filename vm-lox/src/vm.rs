use crate::{chunk::Chunk, ins::Ins, value::Value};

/// The virtual machine.
pub struct Vm {
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Vm {
        Self {
            stack: Vec::with_capacity(256),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), ()> {
        for ins in chunk.code {
            use Ins::*;
            match ins {
                Constant(value) => {
                    self.push(value);
                }
                Negate => match self.pop() {
                    Value::Number(number) => self.push(Value::Number(-number)),
                },
                Return => {
                    let val = self.pop();
                    println!("{val:?}");
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn push(&mut self, value: Value) {
        if self.stack.len() == self.stack.capacity() {
            panic!("Stack overflow");
        }
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
