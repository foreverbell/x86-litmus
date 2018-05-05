use ast::{Proc, Value, Operand};
use ast::{CoreProg, CoreInst};
use state::{State, Terminal};
use std::collections::HashSet;

fn read(prog: CoreProg, processor: Proc, state: State) -> Option<State> {
  let proc_prog = prog.0.get(&processor)?;
  let proc_state = state.procs.get(&processor)?;
  let proc_ip = proc_state.ip?;

  match proc_prog[proc_ip] {
    CoreInst::Read(reg, memloc) => {
      let value = match proc_state.storebuf.get(&memloc) {
        Some(value) => *value,
        None => state.mem.get(&memloc).cloned().unwrap_or_default(),
      };
      let mut ostate = state.clone();
      ostate.procs.get_mut(&processor).unwrap().regs.insert(reg, value);
      Some(ostate)
    },
    _ => None,
  }
}

fn write(prog: CoreProg, processor: Proc, state: State) -> Option<State> {
  None
}

fn flush(prog: CoreProg, processor: Proc, state: State) -> Option<State> {
  None
}

fn fence(prog: CoreProg, processor: Proc, state: State) -> Option<State> {
  None
}

fn lock(prog: CoreProg, processor: Proc, state: State) -> Option<State> {
  None
}

fn unlock(prog: CoreProg, processor: Proc, state: State) -> Option<State> {
  None
}

pub fn run(prog: CoreProg, processor: Proc, init: State) -> Vec<Terminal> {
  vec![]
}
