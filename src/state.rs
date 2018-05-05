use ast::{Value, Proc, MemLoc, Reg};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash)]
pub struct ProcState {
  pub regs: BTreeMap<Reg, Value>,
  pub storebuf: BTreeMap<MemLoc, Value>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct State {
  pub procs: BTreeMap<Proc, ProcState>,
  pub mem: BTreeMap<MemLoc, Value>,
  pub lock_owner: Option<Proc>,
}

pub struct FinalProcState {
  pub regs: BTreeMap<Reg, Value>,
}

pub struct FinalState {
  pub procs: BTreeMap<Proc, FinalProcState>,
  pub mem: BTreeMap<MemLoc, Value>,
}

pub enum FinalStatePredType {
  Forbidden,
  Required,
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

    let mut procs: BTreeMap<Proc, FinalProcState> = BTreeMap::new();
    for (processor, state) in &self.procs {
      procs.insert(*processor, state.finalize()?).unwrap();
    }
    Some(FinalState {procs: procs, mem: self.mem.clone()})
  }
}
