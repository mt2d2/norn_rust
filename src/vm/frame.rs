use vm::ir::Function;
use vm;

pub struct Frame<'a> {
  pub function: &'a Function,
  pub ip: usize,
  pub memory: [vm::Value; 1],
}

impl<'a> Frame<'a> {
  pub fn new(function: &'a Function) -> Frame<'a> {
    Frame{
      function: function,
      ip: 0,
      memory: [0], // TODO you know the memory size required
    }
  }
}
