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
                Add => arithmetic_binary!(self, +),
                Subtract => arithmetic_binary!(self, -),
                Multiply => arithmetic_binary!(self, *),
                Divide => arithmetic_binary!(self, /),
                Return => {
                    let val = self.pop();
                    println!("{val}");
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

macro_rules! arithmetic_binary {
    ($self:expr, $op:tt) => {{
        let b = $self.pop();
        let a = $self.pop();
        use Value::*;
        let out = match (a, b) {
            (Number(a), Number(b)) => Number(a $op b),
        };
        $self.push(out);
    }}
}
use arithmetic_binary;
