extern crate test;

mod vm;

fn main() {
  let program = vm::ir::Program::parse_textual_bytecode(Path::new("test/fib.nornc")).unwrap();
  vm::execute(&program);
}

#[cfg(test)]
mod tests {
  use test::Bencher;

  #[bench]
  fn bench_add_two(b: &mut Bencher) {
    let program = ::vm::ir::Program::parse_textual_bytecode(Path::new("test/fib.nornc")).unwrap();
    b.iter(|| ::vm::execute(&program));
  }
}
