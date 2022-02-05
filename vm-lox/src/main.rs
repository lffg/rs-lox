use vm_lox::{chunk::Chunk, ins::Ins, value::Value, vm::Vm};

fn main() {
    let mut chunk = Chunk::new("test chunk");

    chunk.write(Ins::Constant(Value::Number(3.14)), 2);
    chunk.write(Ins::Negate, 1);
    chunk.write(Ins::Constant(Value::Number(0.14)), 3);
    chunk.write(Ins::Add, 3);

    chunk.write(Ins::Constant(Value::Number(-2.0)), 3);
    chunk.write(Ins::Multiply, 3);

    chunk.write(Ins::Constant(Value::Number(4.0)), 3);
    chunk.write(Ins::Add, 3);

    chunk.write(Ins::Return, 4);

    println!("{:?}", &chunk);

    let mut vm = Vm::new();
    vm.interpret(chunk).unwrap();
}
