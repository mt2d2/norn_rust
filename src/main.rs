mod instructions;
mod functions;

fn main() {
  let op = instructions::Instruction{op: instructions::Opcode::LitInt, arg: 0};
  let func = functions::Function{instructions: vec![op]};

  println!("Function!: {}", func)
}
