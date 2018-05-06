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
      let mut nstate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut nstate);

      nstate.procs.get_mut(&processor).unwrap().regs.insert(
        reg1,
        value,
      );
      Some(nstate)
    },
    CoreInst::Mov2(reg, value) => {
      let mut nstate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut nstate);

      nstate.procs.get_mut(&processor).unwrap().regs.insert(
        reg,
        value,
      );
      Some(nstate)
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
      let mut nstate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut nstate);

      nstate.procs.get_mut(&processor).unwrap().regs.insert(
        reg,
        value,
      );
      Some(nstate)
    },
    _ => None,
  }
}

fn write(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Write(memloc, reg) => {
      let value = proc_state.regs.get(&reg).cloned().unwrap_or_default();
      let mut nstate = state.clone();
      increase_ip(processor, proc_prog.len(), &mut nstate);

      nstate
        .procs
        .get_mut(&processor)
        .unwrap()
        .storebuf
        .push_back((memloc, value));
      Some(nstate)
    },
    _ => None,
  }
}

fn tau(processor: Proc, _: &CoreProg, state: &State) -> Option<State> {
  if state.is_blocked(processor) {
    return None;
  }

  let mut nstate = state.clone();
  {
    let proc_state = nstate.procs.get_mut(&processor)?;

    if let Some((memloc, value)) = proc_state.storebuf.pop_front() {
      nstate.mem.insert(memloc, value);
    } else {
      return None;
    }
  }
  Some(nstate)
}

fn fence(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, proc_state, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Mfence => {
      if proc_state.storebuf.is_empty() {
        let mut nstate = state.clone();
        increase_ip(processor, proc_prog.len(), &mut nstate);

        Some(nstate)
      } else {
        None
      }
    },
    _ => None,
  }
}

fn lock(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, _, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Lock => {
      if state.lock_owner.is_none() {
        let mut nstate = state.clone();
        increase_ip(processor, proc_prog.len(), &mut nstate);

        nstate.lock_owner = Some(processor);
        Some(nstate)
      } else {
        None
      }
    },
    _ => None,
  }
}

fn unlock(processor: Proc, prog: &CoreProg, state: &State) -> Option<State> {
  let (proc_prog, _, proc_ip) = extract(processor, prog, state)?;

  match proc_prog[proc_ip] {
    CoreInst::Unlock => {
      if state.lock_owner == Some(processor) {
        let mut nstate = state.clone();
        increase_ip(processor, proc_prog.len(), &mut nstate);

        nstate.lock_owner = None;
        Some(nstate)
      } else {
        None
      }
    },
    _ => None,
  }
}

pub static NEXT: [fn(Proc, &CoreProg, &State) -> Option<State>; 7] =
  [mov, read, write, tau, fence, lock, unlock];

pub fn run(prog: CoreProg, init: State) -> Vec<Terminal> {
  let processors: Vec<Proc> = prog.0.keys().cloned().collect();
  let mut queue: VecDeque<State> = VecDeque::new();
  let mut hashtbl: HashSet<State> = HashSet::new();
  let mut result: Vec<Terminal> = Vec::new();

  queue.push_back(init.clone());
  hashtbl.insert(init);

  while !queue.is_empty() {
    let front = queue.pop_front().unwrap();

    if front.is_final() {
      result.push(front.finalize().unwrap());
      continue;
    }
    for processor in &processors {
      for next in NEXT.into_iter() {
        if let Some(nstate) = next(*processor, &prog, &front) {
          if hashtbl.contains(&nstate) {
            continue;
          }
          queue.push_back(nstate.clone());
          hashtbl.insert(nstate);
        }
      }
    }
  }
  result
}
