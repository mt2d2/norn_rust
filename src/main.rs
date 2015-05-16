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
    let program = vm::ir::programs::Program::parse_textual_bytecode(Path::new(file));

    match program  {
        Ok(program) => vm::execute(&program),
        Err(e) => match e {
            vm::ir::programs::ParseError::Io(e)        => panic!("io error: {}", e),
            vm::ir::programs::ParseError::BadSplit     => panic!("bad split detected"),
            vm::ir::programs::ParseError::BadInt       => panic!("integer parse failed"),
            vm::ir::programs::ParseError::BadOpcode    => panic!("unknown opcode encounted"),
        }
    }
}
