extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Inst, Pred, PredType, Prog};
use x86_litmus::ast::{Proc, Operand, Reg, Value, MemLoc};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn n4b() {
  let x = Operand::MemLoc(MemLoc("x"));
  let eax = Operand::Reg(Reg::Eax);
  let ecx = Operand::Reg(Reg::Ecx);
  let one = Operand::Imm(Value(1));
  let two = Operand::Imm(Value(2));

  let p0 = Proc(0);
  let i0 = vec![Inst::Mov(eax, x), Inst::Mov(x, one)];

  let p1 = Proc(1);
  let i1 = vec![Inst::Mov(ecx, x), Inst::Mov(x, two)];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(p0, i0);
  prog.insert(p1, i1);

  let pred = Pred::And(vec![
    Pred::Reg(p0, Reg::Eax, Value(2)),
    Pred::Reg(p1, Reg::Ecx, Value(1)),
  ]);

  assert!(litmus(
    "n4b",
    &Prog(prog),
    State::new(&vec![p0, p1]),
    &pred,
    PredType::Forbidden,
  ));
}
