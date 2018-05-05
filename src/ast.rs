use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Value(pub i32);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Proc(pub u32);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct MemLoc(pub &'static str);

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

pub struct SurfaceProg(pub HashMap<Proc, Vec<SurfaceInstr>>);
pub struct CoreProg(pub HashMap<Proc, Vec<CoreInstr>>);

pub struct ProcState {
  pub regs: HashMap<Reg, Value>,
  pub storebuf: HashMap<MemLoc, Value>,
}

pub struct State {
  pub procs: HashMap<Proc, ProcState>,
  pub mem: HashMap<MemLoc, Value>,
  pub lock_owner: Option<Proc>,
}

pub struct FinalProcState {
  pub regs: HashMap<Reg, Value>,
}

pub struct FinalState {
  pub procs: HashMap<Proc, FinalProcState>,
  pub mem: HashMap<MemLoc, Value>,
}

impl ProcState {
  pub fn finalize(&self) -> Option<FinalProcState> {
    if !self.storebuf.is_empty() {
      return None;
    }
    Some(FinalProcState {regs: self.regs.clone()})
  }
}

impl State {
  pub fn finalize(&self) -> Option<FinalState> {
    if self.lock_owner.is_some() {
      return None;
    }

    let mut procs: HashMap<Proc, FinalProcState> = HashMap::new();
    for (processor, state) in &self.procs {
      procs.insert(*processor, state.finalize()?).unwrap();
    }
    Some(FinalState {procs: procs, mem: self.mem.clone()})
  }
}
