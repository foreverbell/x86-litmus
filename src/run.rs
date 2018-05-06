use ast::{CoreProg, CoreInst};
use ast::{Proc, Value, MemLoc};
use state::{State, ProcState, Terminal};
use std::collections::HashSet;
use std::collections::VecDeque;

fn extract<'a, 'b: 'a>(
  processor: Proc,
  prog: &'b CoreProg,
  state: &'b State,
) -> Option<(&'a Vec<CoreInst>, &'a ProcState, usize)> {
  let proc_prog = prog.0.get(&processor)?;
  let proc_state = state.procs.get(&processor)?;
  let proc_ip = proc_state.ip?;

  Some((proc_prog, proc_state, proc_ip))
}

fn find_lastest_write(
  storebuf: &VecDeque<(MemLoc, Value)>,
  memloc: MemLoc,
) -> Option<Value> {
  for i in (0..storebuf.len()).rev() {
    let item = storebuf.get(i).unwrap();
    if item.0 == memloc {
      return Some(item.1);
    }
  }
  None
}

fn increase_ip(processor: Proc, prog_size: usize, state: &mut State) {
  let ip: &mut Option<usize> = &mut state.procs.get_mut(&processor).unwrap().ip;

  if ip.is_some() {
    let new_ip = ip.unwrap() + 1;

    if new_ip < prog_size {
      *ip = Some(new_ip);
    } else {
      *ip = None;
    }
  }
}

fn mov(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Mov1(reg1, reg2) => {
      let value = proc_state.regs.get(&reg2).cloned().unwrap_or_default();
      let mut ostate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut ostate);

      ostate.procs.get_mut(&processor).unwrap().regs.insert(
        reg1,
        value,
      );
      Some(ostate)
    },
    CoreInst::Mov2(reg, value) => {
      let mut ostate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut ostate);

      ostate.procs.get_mut(&processor).unwrap().regs.insert(
        reg,
        value,
      );
      Some(ostate)
    },
    _ => None,
  }
}

fn read(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Read(reg, memloc) => {
      if state.is_blocked(processor) {
        return None;
      }

      let value = match find_lastest_write(&proc_state.storebuf, memloc) {
        Some(value) => value,
        None => state.mem.get(&memloc).cloned().unwrap_or_default(),
      };
      let mut ostate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut ostate);

      ostate.procs.get_mut(&processor).unwrap().regs.insert(
        reg,
        value,
      );
      Some(ostate)
    },
    _ => None,
  }
}

fn write(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Write(memloc, reg) => {
      let value = proc_state.regs.get(&reg).cloned().unwrap_or_default();
      let mut ostate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut ostate);

      ostate
        .procs
        .get_mut(&processor)
        .unwrap()
        .storebuf
        .push_back((memloc, value));
      Some(ostate)
    },
    _ => None,
  }
}

fn flush_one(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  if state.is_blocked(processor) {
    return None;
  }

  let mut ostate = state.clone();
  {
    let proc_state = ostate.procs.get_mut(&processor)?;

    if let Some((memloc, value)) = proc_state.storebuf.pop_front() {
      ostate.mem.insert(memloc, value);
    } else {
      return None;
    }
  }
  Some(ostate)
}

fn fence(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Mfence => {
      if proc_state.storebuf.is_empty() {
        let mut ostate = state.clone();
        increase_ip(processor, proc_prog.len(), &mut ostate);

        Some(ostate)
      } else {
        None
      }
    },
    _ => None,
  }
}

fn lock(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Lock => {
      if state.lock_owner.is_none() {
        let mut ostate = state.clone();
        increase_ip(processor, proc_prog.len(), &mut ostate);

        ostate.lock_owner = Some(processor);
        Some(ostate)
      } else {
        None
      }
    },
    _ => None,
  }
}

fn unlock(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Lock => {
      if state.lock_owner == Some(processor) {
        let mut ostate = state.clone();
        increase_ip(processor, proc_prog.len(), &mut ostate);

        ostate.lock_owner = None;
        Some(ostate)
      } else {
        None
      }
    },
    _ => None,
  }
}

pub fn run(processor: Proc, prog: CoreProg, init: State) -> Vec<Terminal> {
  vec![]
}
