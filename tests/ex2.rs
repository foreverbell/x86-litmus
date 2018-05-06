extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Inst, Pred, PredType, Prog};
use x86_litmus::ast::{Proc, Operand, Reg, Value, MemLoc};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn ex2() {
  let x = Operand::MemLoc(MemLoc("x"));
  let y = Operand::MemLoc(MemLoc("y"));
  let eax = Operand::Reg(Reg::Eax);
  let ebx = Operand::Reg(Reg::Ebx);
  let one = Operand::Imm(Value(1));

  let p0 = Proc(0);
  let i0 = vec![Inst::Mov(eax, x), Inst::Mov(y, one)];

  let p1 = Proc(1);
  let i1 = vec![Inst::Mov(ebx, y), Inst::Mov(x, one)];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(p0, i0);
  prog.insert(p1, i1);

  let pred = Pred::And(vec![
    Pred::Reg(p0, Reg::Eax, Value(1)),
    Pred::Reg(p1, Reg::Ebx, Value(1)),
  ]);

  assert!(litmus(
    "ex2",
    &Prog(prog),
    State::new(&vec![p0, p1]),
    &pred,
    PredType::Forbidden,
  ));
}
