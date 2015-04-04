use std::env;
use std::path::Path;

mod vm;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: norn_rust file.nornc");
        return;
    }

    let file = &args.nth(1).unwrap();
    let program = vm::ir::Program::parse_textual_bytecode(Path::new(file)).unwrap();

    vm::execute(&program);
}
