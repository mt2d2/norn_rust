#[deriving(Show)]
pub enum Opcode {
  LitInt,
  LitChar,
  StoreInt,
  StoreChar,
  LoadInt,
  LoadChar,
  PrintInt,
  PrintChar,
  LeInt,
  AddInt,
  SubInt,
  Tjmp,
  Fjmp,
  Ujmp,
  Call,
  Rtrn,
  Label,
}

impl Opcode {
  pub fn parse(string: &str) -> Option<Opcode> {
    match string {
      "LIT_INT" => Some(Opcode::LitInt),
      "LIT_CHAR" => Some(Opcode::LitChar),
      "STORE_INT" => Some(Opcode::StoreInt),
      "STORE_CHAR" => Some(Opcode::StoreChar),
      "LOAD_INT" => Some(Opcode::LoadInt),
      "LOAD_CHAR" => Some(Opcode::LoadChar),
      "PRINT_INT" => Some(Opcode::PrintInt),
      "PRINT_CHAR" => Some(Opcode::PrintChar),
      "LE_INT"  => Some(Opcode::LeInt),
      "ADD_INT" => Some(Opcode::AddInt),
      "SUB_INT" => Some(Opcode::SubInt),
      "TJMP" => Some(Opcode::Tjmp),
      "FJMP" => Some(Opcode::Fjmp),
      "UJMP" => Some(Opcode::Ujmp),
      "CALL" => Some(Opcode::Call),
      "RTRN" => Some(Opcode::Rtrn),
      "LABEL" => Some(Opcode::Label),

      _ => None
    }
  }

  pub fn is_jmp(&self) -> bool {
    match *self {
      Opcode::Tjmp | Opcode::Fjmp | Opcode::Ujmp => true,
      _ => false,
    }
  }
}

#[deriving(Show)]
pub struct Instruction {
  pub op: Opcode,
  pub arg: int,
}
