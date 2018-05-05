use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Value(pub i32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Proc(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct MemLoc(pub &'static str);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Reg {
  EAX,
  EBX,
  ECX,
  EDX,
}

#[derive(Clone, Copy)]
pub enum SrcOperand {
  Imm(Value),
  Reg(Reg),
  MemLoc(MemLoc),
}

#[derive(Clone, Copy)]
pub enum DstOperand {
  Reg(Reg),
  MemLoc(MemLoc),
}

#[derive(Clone, Copy)]
pub enum SurfaceInstr {
  MOV(DstOperand, SrcOperand),
  XCHG(DstOperand, DstOperand),
  MFENCE,
}

#[derive(Clone, Copy)]
pub enum CoreInstr {
  MOV(DstOperand, SrcOperand),
  XCHG(DstOperand, DstOperand),
  MFENCE,
  LOCK,
  UNLOCK,
}

pub struct SurfaceProg(pub BTreeMap<Proc, Vec<SurfaceInstr>>);

pub struct CoreProg(pub BTreeMap<Proc, Vec<CoreInstr>>);
