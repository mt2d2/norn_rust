#[deriving(Show)]
pub enum Opcode {
  LitInt,
}

#[deriving(Show)]
pub struct Instruction {
  pub op: Opcode,
  pub arg: int,
}
