use std::env;
use std::fs::File;


mod vm;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: norn_rust file.nornc");
        return;
    }

    let path = &args.nth(1).unwrap();
    let file = File::open(path).ok().expect("could not open file");
    let program = vm::ir::programs::Program::parse_textual_bytecode(file);

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
