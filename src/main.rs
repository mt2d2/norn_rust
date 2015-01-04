extern crate test;

use std::os;

mod vm;

fn main() {
  let args = os::args();
  let args_tail = args.tail();
  if args_tail.len() != 1 {
    println!("Usage: norn_rust file.nornc");
    return;
  }

  let file = &args_tail[0];
  let program = vm::ir::Program::parse_textual_bytecode(Path::new(file)).unwrap();

  vm::execute(&program);
}

#[cfg(test)]
mod tests {
  use test::Bencher;

  #[bench]
  fn bench_interpret_fib(b: &mut Bencher) {
    let program = ::vm::ir::Program::parse_textual_bytecode(Path::new("test/fib.nornc")).unwrap();
    b.iter(|| ::vm::execute(&program));
  }
}
