extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Inst, Pred, PredType, Prog};
use x86_litmus::ast::{Proc, Operand, Reg, Value, MemLoc};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn ex9() {
  let x = Operand::MemLoc(MemLoc("x"));
  let y = Operand::MemLoc(MemLoc("y"));
  let eax = Operand::Reg(Reg::Eax);
  let ebx = Operand::Reg(Reg::Ebx);
  let ecx = Operand::Reg(Reg::Ecx);
  let edx = Operand::Reg(Reg::Edx);

  let p0 = Proc(0);
  let i0 = vec![Inst::Xchg(x, eax), Inst::Mov(ebx, y)];

  let p1 = Proc(1);
  let i1 = vec![Inst::Xchg(y, ecx), Inst::Mov(edx, x)];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(p0, i0);
  prog.insert(p1, i1);

  let mut init = State::new(&vec![p0, p1]);
  init.procs.get_mut(&p0).unwrap().regs.insert(
    Reg::Eax,
    Value(1),
  );
  init.procs.get_mut(&p1).unwrap().regs.insert(
    Reg::Ecx,
    Value(1),
  );

  let pred = Pred::And(vec![
    Pred::Reg(p0, Reg::Ebx, Value(0)),
    Pred::Reg(p1, Reg::Edx, Value(0)),
  ]);

  assert!(litmus("ex9", &Prog(prog), init, &pred, PredType::Forbidden));
}
