mod vm;

fn main() {
  let program = vm::ir::Program::parse_textual_bytecode(Path::new("test/fib.nornc")).unwrap();

  println!("{}", program);
  vm::execute(&program);
}
