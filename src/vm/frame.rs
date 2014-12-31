use vm::ir::Function;

pub struct Frame<'a> {
  pub stack: Vec<int>,
  pub function: &'a Function,
  pub ip: uint,
  pub memory: Vec<int>,
}

impl<'a> Frame<'a> {
  pub fn new(function: &'a Function) -> Frame<'a> {
    Frame{
      stack: vec![],
      function: function,
      ip: 0,
      memory: vec![0], // TODO you know the memory size required
    }
  }
}
