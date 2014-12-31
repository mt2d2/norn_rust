mod instructions;
mod functions;
mod programs;

use programs;

fn main() {
  let program = Program::parse_textual_bytecode(Path::new("test/fib.nornc")).unwrap();
  println!("{}", program);
}
