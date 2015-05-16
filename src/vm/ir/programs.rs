use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use vm::ir::instructions::{Opcode, Instruction};
use vm::ir::functions::Function;
use vm;

use std::num;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    BadSplit,
    BadInt,
    BadOpcode,
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match *self {
            ParseError::Io(_) => "io error".to_string(),
            ParseError::BadSplit => "bad split".to_string(),
            ParseError::BadInt => "bad int".to_string(),
            ParseError::BadOpcode => "Bad opcode".to_string()
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(_: num::ParseIntError) -> ParseError {
        ParseError::BadInt
    }
}

impl Program {
    pub fn parse_textual_bytecode<T: io::Read>(input: T) -> Result<Program, ParseError> {
        let mut program = Program{functions: vec![Function{instructions: vec![]}]};
        let file = io::BufReader::new(input);

        let mut instruction_count = 0;
        let mut jump_targets = HashMap::new();

        for line in file.lines() {
            let line_unwrap = try!(line);

            if !line_unwrap.chars().all(|c| c.is_whitespace()) {
                let mut it = line_unwrap.split(" ").map(|s| s.trim());
                let opcode_it = try!(it.next().ok_or(ParseError::BadSplit));
                let op = try!(Opcode::parse(opcode_it).ok_or(ParseError::BadOpcode));
                let arg_it = try!(it.next().ok_or(ParseError::BadSplit));
                let arg = try!(arg_it.parse::<vm::Value>());

                match op {
                    Opcode::Label => {
                        jump_targets.insert(arg, instruction_count);
                    },
                    _ => {
                        instruction_count += 1;
                        let new_instruction = Instruction{op: op, arg: arg};
                        program.functions.last_mut().unwrap().instructions.push(new_instruction);
                    }
                }
            } else {
                // before moving onto a new function, normalize any instructions jump target
                for instruction in program.functions.last_mut().unwrap().instructions.iter_mut() {
                if instruction.op.is_jmp() {
                    instruction.arg = jump_targets[&instruction.arg] as vm::Value;
                }
            }

            instruction_count = 0;
            jump_targets.clear();
            program.functions.push(Function{instructions: vec![]});
            }
        }

        Ok(program)
    }
}
