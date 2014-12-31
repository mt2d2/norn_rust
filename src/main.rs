mod instructions;
mod functions;
mod programs;

fn main() {
  let program = programs::Program::parse_textual_bytecode(Path::new("test/fib.nornc")).unwrap();
  println!("{}", program);
}
