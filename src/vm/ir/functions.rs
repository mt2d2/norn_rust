use vm::ir::instructions::Instruction;

#[derive(Debug)]
pub struct Function {
  pub instructions: Vec<Instruction>,
}
