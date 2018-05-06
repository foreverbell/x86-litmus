use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Value(pub i32);

impl Default for Value {
  fn default() -> Self {
    Value(0)
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Proc(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct MemLoc(pub &'static str);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Reg {
  Eax,
  Ebx,
  Ecx,
  Edx,
}

#[derive(Clone, Copy)]
pub enum Operand {
  Imm(Value),
  Reg(Reg),
  MemLoc(MemLoc),
}

#[derive(Clone, Copy)]
pub enum Inst {
  Mov(Operand, Operand),
  Xchg(Operand, Operand),
  Mfence,
}

#[derive(Clone, Copy)]
pub enum CoreInst {
  // read from memory to register
  Read(Reg, MemLoc),
  // write to memory from register
  Write1(MemLoc, Reg),
  // write to memory with an immediate value
  Write2(MemLoc, Value),
  // move from register to register
  Mov1(Reg, Reg),
  // move immediate value to register
  Mov2(Reg, Value),
  // exchange value between register and memory
  Xchg(Reg, MemLoc),
  // memory fence
  Mfence,
  // lock bus
  Lock,
  // unlock bus
  Unlock,
}

pub struct Prog(pub BTreeMap<Proc, Vec<Inst>>);
pub struct CoreProg(pub BTreeMap<Proc, Vec<CoreInst>>);

pub enum Pred {
  Reg(Proc, Reg, Value),
  MemLoc(MemLoc, Value),
  Not(Box<Pred>),
  And(Vec<Pred>),
}
