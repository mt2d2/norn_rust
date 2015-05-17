use std::fs::File;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

mod vm;

static USAGE: &'static str = "
Usage: norn_rust <file>
       norn_rust (--help | --version)

Options:
    -h, --help       Show this message.
    -v, --version    Show version of norn_rust.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: String,
    flag_help: bool,
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                              .and_then(|d| d.decode())
                              .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        println!("{}", USAGE);
        return;
    }

    if args.flag_version {
        println!("norn_rust 0.0.3");
        return;
    }

    File::open(args.arg_file)
        .map_err(|err| err.to_string())
        .and_then(|file| {
                vm::ir::programs::Program::parse_textual_bytecode(file)
                .map_err(|err| err.to_string())
            })
        .and_then(|program| Ok(vm::execute(&program)))
        .unwrap();
}
