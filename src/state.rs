use ast::{Value, Proc, MemLoc, Reg, Pred};
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
  pub fn new() -> Self {
    ProcState {
      regs: BTreeMap::new(),
      ip: Some(0),
      storebuf: VecDeque::new(),
    }
  }

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
  pub fn new(processors: &Vec<Proc>) -> Self {
    let mut procs = BTreeMap::new();

    for processor in processors {
      procs.insert(*processor, ProcState::new());
    }

    State {
      procs: procs,
      mem: BTreeMap::new(),
      lock_owner: None,
    }
  }

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
      procs.insert(*processor, state.finalize().unwrap());
    }
    Some(Terminal {
      procs: procs,
      mem: self.mem.clone(),
    })
  }
}

impl Terminal {
  pub fn satisfy(&self, pred: &Pred) -> bool {
    match pred {
      &Pred::Reg(processor, reg, value) => {
        let proc_terminal: &ProcTerminal = self.procs.get(&processor).unwrap();
        value == proc_terminal.regs.get(&reg).cloned().unwrap_or_default()
      },
      &Pred::MemLoc(memloc, value) => {
        value == self.mem.get(&memloc).cloned().unwrap_or_default()
      },
      &Pred::And(ref preds) => {
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
