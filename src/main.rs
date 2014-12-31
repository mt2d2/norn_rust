mod instructions;
mod functions;
mod programs;

fn main() {
  let program = programs::parse_program(Path::new("fib.nornc")).unwrap();
  println!("{}", program);
}
