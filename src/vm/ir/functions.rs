use vm::ir::instructions::Instruction;

#[deriving(Show)]
pub struct Function {
  pub instructions: Vec<Instruction>,
}
