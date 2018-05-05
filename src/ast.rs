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
pub enum SurfaceInst {
  Mov(Operand, Operand),
  Xchg(Operand, Operand),
  Mfence,
}

#[derive(Clone, Copy)]
pub enum CoreInst {
  Read(Reg, MemLoc),   // read from memory to register
  Write(MemLoc, Reg),  // write to memory from register
  Mov1(Reg, Reg),      // move from register to register
  Mov2(Reg, Value),    // move immediate value to register
  Xchg(Reg, MemLoc),   // exchange value between register and memory
  Mfence,              // memory fence
  Lock,                // lock bus
  Unlock,              // unlock bus
}

pub struct SurfaceProg(pub BTreeMap<Proc, Vec<SurfaceInst>>);
pub struct CoreProg(pub BTreeMap<Proc, Vec<CoreInst>>);
