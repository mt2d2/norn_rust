mod instructions;
mod functions;
mod programs;

fn main() {
  let op = instructions::Instruction{op: instructions::Opcode::LitInt, arg: 0};
  let func = functions::Function{instructions: vec![op]};
  let program = programs::Program{functions: vec![func]};

  println!("Program!: {}", program)
}
