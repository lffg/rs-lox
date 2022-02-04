use vm_lox::{chunk::Chunk, ins::Ins, value::Value, vm::Vm};

fn main() {
    let mut chunk = Chunk::new("test chunk");
    chunk.write(Ins::Constant(Value::Number(5.)), 1);
    chunk.write(Ins::Constant(Value::Number(10.)), 1);
    chunk.write(Ins::Constant(Value::Number(15.)), 1);
    chunk.write(Ins::Return, 123);

    let mut vm = Vm::new();
    vm.interpret(chunk).unwrap();
}
