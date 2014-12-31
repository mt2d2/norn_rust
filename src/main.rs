mod instructions;

fn main() {
  let op = instructions::Instruction{op: instructions::Opcode::LitInt, arg: 0};

  println!("Hello, world!, Here is the op: {}", op);

}
