use ast::{Value, Proc, MemLoc, Reg};
use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ProcState {
  pub regs: BTreeMap<Reg, Value>,
  // None if program is terminated (but may still have uncommitted writes in
  // storebuf).
  pub ip: Option<usize>,
  pub storebuf: VecDeque<(MemLoc, Value)>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct State {
  pub procs: BTreeMap<Proc, ProcState>,
  pub mem: BTreeMap<MemLoc, Value>,
  pub lock_owner: Option<Proc>,
}

#[derive(Clone, Default)]
pub struct ProcTerminal {
  pub regs: BTreeMap<Reg, Value>,
}

pub struct Terminal {
  pub procs: BTreeMap<Proc, ProcTerminal>,
  pub mem: BTreeMap<MemLoc, Value>,
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

pub enum TerminalPred {
  Reg(Proc, Reg, Value),
  MemLoc(MemLoc, Value),
  Not(Box<TerminalPred>),
  And(Vec<TerminalPred>),
}

impl Terminal {
  pub fn satisfy(&self, pred: &TerminalPred) -> bool {
    match pred {
      &TerminalPred::Reg(processor, reg, value) => {
        let proc_terminal: ProcTerminal =
          self.procs.get(&processor).cloned().unwrap_or_default();
        value == proc_terminal.regs.get(&reg).cloned().unwrap_or_default()
      },
      &TerminalPred::MemLoc(memloc, value) => {
        value == self.mem.get(&memloc).cloned().unwrap_or_default()
      },
      &TerminalPred::Not(ref pred) => !self.satisfy(pred),
      &TerminalPred::And(ref preds) => {
        for pred in preds {
          if !self.satisfy(pred) {
            return false;
          }
        }
        true
      },
    }
  }
}
