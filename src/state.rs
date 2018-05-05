use ast::{Value, Proc, MemLoc, Reg};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ProcState {
  pub regs: BTreeMap<Reg, Value>,
  pub ip: Option<usize>,  // None if program is terminated.
  pub storebuf: BTreeMap<MemLoc, Value>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct State {
  pub procs: BTreeMap<Proc, ProcState>,
  pub mem: BTreeMap<MemLoc, Value>,
  pub lock_owner: Option<Proc>,
}

pub struct ProcTerminal {
  pub regs: BTreeMap<Reg, Value>,
}

pub struct Terminal {
  pub procs: BTreeMap<Proc, ProcTerminal>,
  pub mem: BTreeMap<MemLoc, Value>,
}

pub enum TerminalPred {
  Reg(Proc, Reg, Value),
  MemLoc(MemLoc, Value),
  Not(Box<TerminalPred>),
  And(Vec<TerminalPred>),
}

impl ProcState {
  pub fn is_final(&self) -> bool {
    self.storebuf.is_empty() && self.ip.is_none()
  }

  pub fn finalize(&self) -> Option<ProcTerminal> {
    if !self.is_final() {
      return None;
    }
    Some(ProcTerminal { regs: self.regs.clone() })
  }
}

impl State {
  pub fn is_blocked(&self, processor: Proc) -> bool {
    match self.lock_owner {
      None => false,
      Some(processor2) => processor != processor2,
    }
  }

  pub fn is_final(&self) -> bool {
    if self.lock_owner.is_some() {
      return false;
    }
    for state in self.procs.values() {
      if !state.is_final() {
        return false;
      }
    }
    true
  }

  pub fn finalize(&self) -> Option<Terminal> {
    if !self.is_final() {
      return None;
    }

    let mut procs: BTreeMap<Proc, ProcTerminal> = BTreeMap::new();
    for (processor, state) in &self.procs {
      procs.insert(*processor, state.finalize().unwrap()).unwrap();
    }
    Some(Terminal {
      procs: procs,
      mem: self.mem.clone(),
    })
  }
}
