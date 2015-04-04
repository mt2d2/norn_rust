use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{Error, BufRead, BufReader};

use vm::ir::instructions::{Opcode, Instruction};
use vm::ir::functions::Function;
use vm;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub enum ParseError {
    Io(Error),
    BadSplit,
    BadInt,
    BadOpcode,
}

impl Program {
    pub fn parse_textual_bytecode(path: &Path) -> Result<Program, ParseError> {
        let mut program = Program{functions: vec![Function{instructions: vec![]}]};
        let file = File::open(path).unwrap();
        let file = BufReader::new(file);

        let mut instruction_count = 0;
        let mut jump_targets = HashMap::new();

        for line in file.lines() {
            let line_unwrap = try!(match line {
                Err(e) => {
                    Err(ParseError::Io(e))
                },
                Ok(v) => {
                    Ok(v)
                }
            });

            if !line_unwrap.chars().all(|c| c.is_whitespace()) {
                let mut it = line_unwrap.split(" ").map(|s| s.trim());
                let opcode_it = try!(it.next().ok_or(ParseError::BadSplit));
                let op = try!(Opcode::parse(opcode_it).ok_or(ParseError::BadOpcode));
                let arg_it = try!(it.next().ok_or(ParseError::BadSplit));
                let arg = try!(match arg_it.parse::<vm::Value>() {
                    Err(_) => {
                        Err(ParseError::BadInt)
                    },
                    Ok(v) => {
                        Ok(v)
                    }
                });

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
