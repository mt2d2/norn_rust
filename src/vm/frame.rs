use vm::ir::Function;

struct Frame {
  stack: Vec<int>,
  // function: &Function,
  ip: uint,
  memory: Vec<int>,
}
