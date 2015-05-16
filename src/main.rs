use std::env;
use std::fs::File;

mod vm;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: norn_rust file.nornc");
        return;
    }

    File::open(args.nth(1).unwrap())
        .map_err(|err| err.to_string())
        .and_then(|file| {
                vm::ir::programs::Program::parse_textual_bytecode(file)
                .map_err(|err| err.to_string())
            })
        .and_then(|program| Ok(vm::execute(&program)))
        .unwrap();
}
