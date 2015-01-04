use vm::ir::instructions::Instruction;

#[derive(Show)]
pub struct Function {
  pub instructions: Vec<Instruction>,
}
