use std::collections::HashMap;
use std::error::FromError;
use std::io::{BufferedReader, File, IoError};

use instructions::{Opcode, Instruction};
use functions::{Function};

#[deriving(Show)]
pub struct Program {
  pub functions: Vec<::functions::Function>,
}

#[deriving(Show)]
enum ParseError {
  Io(IoError),
  BadSplit,
  BadInt,
  BadOpcode,
}

impl FromError<IoError> for ParseError {
  fn from_error(err: IoError) -> ParseError {
    ParseError::Io(err)
  }
}

pub fn parse_program(path: Path) -> Result<Program, ParseError> {
  let mut program = Program{functions: vec![Function{instructions: vec![]}]};
  let mut file = BufferedReader::new(File::open(&path));

  let mut instruction_count = 0u;
  let mut jump_targets = HashMap::new();

  for line in file.lines() {
    let line_unwrap = try!(line);
    if !line_unwrap.chars().all(|c| c.is_whitespace()) {
      let mut it = line_unwrap.split_str(" ").map(|s| s.trim());
      let opcode_it = try!(it.next().ok_or(ParseError::BadSplit));
      let op = try!(Opcode::parse(opcode_it).ok_or(ParseError::BadOpcode));
      let arg_it = try!(it.next().ok_or(ParseError::BadSplit));
      let arg = try!(arg_it.parse::<int>().ok_or(ParseError::BadInt));

      match op {
        Opcode::Label => {
          jump_targets.insert(arg, instruction_count);
        },
        _ => {
          instruction_count += 1;
          let new_instruction = Instruction{op: op, arg: arg};

          let size = program.functions.len() - 1;
          program.functions[size].instructions.push(new_instruction);
        }
      }
    } else {
      // before moving onto a new function, normalize any instructions jump target
      let size = program.functions.len() - 1;
      for instruction in program.functions[size].instructions.iter_mut() {
        if instruction.op.is_jmp() {
          instruction.arg = jump_targets[instruction.arg] as int;
        }
      }

      instruction_count = 0;
      jump_targets.clear();
      program.functions.push(Function{instructions: vec![]});
    }
  }

  Ok(program)
}
