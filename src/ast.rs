use std::collections::HashMap;

pub type Proc = u32;
pub type Value = i32;
pub type MemLoc = &'static str;

#[derive(Clone, Copy)]
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

pub struct ProcState {
  pub regs: HashMap<Reg, Value>,
  pub storebuf: HashMap<MemLoc, Value>,
}

pub struct State {
  pub procs: HashMap<Proc, ProcState>,
  pub mem: HashMap<MemLoc, Value>,
  pub lock_owner: Option<Proc>,
}
