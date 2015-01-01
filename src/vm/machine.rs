use std::mem;

use vm::ir::Program;
use vm::ir::instructions::Opcode;
use vm::Frame;

pub fn execute(program: &Program) {
  let main = program.functions.last().expect("No functions!");

  let mut frames = Vec::with_capacity(256);
  let mut frame = &mut Frame::new(main);

  let mut stack = Vec::with_capacity(256);

  loop {
    let instr = &frame.function.instructions[frame.ip];
    frame.ip += 1;

    match instr.op {
      Opcode::LitInt => {
        stack.push(instr.arg);
      },

      Opcode::LitChar => {
        stack.push(instr.arg);
      },

      Opcode::StoreInt => {
        frame.memory[instr.arg as uint] = stack.pop().unwrap();
      },

      Opcode::StoreChar => {
        frame.memory[instr.arg as uint] = stack.pop().unwrap();
      },

      Opcode::LoadInt => {
        stack.push(frame.memory[instr.arg as uint]);
      },

      Opcode::LoadChar => {
        stack.push(frame.memory[instr.arg as uint]);
      },

      Opcode::PrintInt => {
        print!("{}", stack.pop().unwrap());
      },

      Opcode::PrintChar => {
        print!("{}", stack.pop().unwrap() as u8 as char);
      },

      Opcode::LeInt => {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(if a < b { 1 } else { 0 });
      },

      Opcode::AddInt => {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(a + b);
      },

      Opcode::SubInt => {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(a - b);
      },

      Opcode::Tjmp => {
        if stack.pop().unwrap() == 1 {
          frame.ip = instr.arg as uint;
        }
      },

      Opcode::Fjmp => {
        if stack.pop().unwrap() == 0 {
          frame.ip = instr.arg as uint;
        }
      },

      Opcode::Ujmp => {
        frame.ip = instr.arg as uint;
      },

      Opcode::Call => {
        let mut nframe = Frame::new(&program.functions[instr.arg as uint]);
        mem::swap(frame, &mut nframe);
        frames.push(nframe);
      },

      Opcode::Rtrn => {
        if frames.is_empty() {
          break;
        } else {
          let mut nframe = frames.pop().unwrap();
          mem::swap(frame, &mut nframe);
        }
      },

      Opcode::Label => {
        panic!("Unknown label encounted!");
      },
    }
  }
}
