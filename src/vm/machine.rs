use std::mem;

use vm::ir::Program;
use vm::ir::instructions::Opcode;
use vm::Frame;

pub fn execute(program: &Program) {
  let main = program.functions.last().expect("No functions!");

  let mut frames = Vec::with_capacity(256);
  let mut frame = &mut Frame::new(main);

  loop {
    let ref instr = frame.function.instructions[frame.ip];
    frame.ip += 1;

    match instr.op {
      Opcode::LitInt => {
        frame.stack.push(instr.arg);
      },

      Opcode::LitChar => {
        frame.stack.push(instr.arg);
      },

      Opcode::StoreInt => {
        frame.memory[instr.arg as uint] = frame.stack.pop().unwrap();
      },

      Opcode::StoreChar => {
        frame.memory[instr.arg as uint] = frame.stack.pop().unwrap();
      },

      Opcode::LoadInt => {
        frame.stack.push(frame.memory[instr.arg as uint]);
      },

      Opcode::LoadChar => {
        frame.stack.push(frame.memory[instr.arg as uint]);
      },

      Opcode::PrintInt => {
        print!("{}", frame.stack.pop().unwrap());
      },

      Opcode::PrintChar => {
        print!("{}", frame.stack.pop().unwrap() as u8 as char);
      },

      Opcode::LeInt => {
        let a = frame.stack.pop().unwrap();
        let b = frame.stack.pop().unwrap();
        frame.stack.push(if a < b { 1 } else { 0 });
      },

      Opcode::AddInt => {
        let a = frame.stack.pop().unwrap();
        let b = frame.stack.pop().unwrap();
        frame.stack.push(a + b);
      },

      Opcode::SubInt => {
        let a = frame.stack.pop().unwrap();
        let b = frame.stack.pop().unwrap();
        frame.stack.push(a - b);
      },

      Opcode::Tjmp => {
        if frame.stack.pop().unwrap() == 1 {
          frame.ip = instr.arg as uint;
        }
      },

      Opcode::Fjmp => {
        if frame.stack.pop().unwrap() == 0 {
          frame.ip = instr.arg as uint;
        }
      },

      Opcode::Ujmp => {
        frame.ip = instr.arg as uint;
      },

      Opcode::Call => {
        let arg = frame.stack.pop().unwrap();
        // println!("arg was {} ", arg);

        let mut nframe = Frame::new(&program.functions[instr.arg as uint]);
        mem::swap(frame, &mut nframe);
        frames.push(nframe);

        frame.stack.push(arg);
      },

      Opcode::Rtrn => {
        if frames.is_empty() {
          break;
        } else {
          let ret = frame.stack.pop();

          let mut nframe = frames.pop().unwrap();
          mem::swap(frame, &mut nframe);

          match ret {
            Some(v) => { frame.stack.push(v); },
            None => { },
          }

        }
      },

      Opcode::Label => {
        panic!("Unknown label encounted!");
      },
    }
  }
}
