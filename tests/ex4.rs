extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Inst, Pred, PredType, Prog};
use x86_litmus::ast::{Proc, Operand, Reg, Value, MemLoc};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn ex4() {
  let x = Operand::MemLoc(MemLoc("x"));
  let eax = Operand::Reg(Reg::Eax);
  let one = Operand::Imm(Value(1));

  let p0 = Proc(0);
  let i0 = vec![Inst::Mov(x, one), Inst::Mov(eax, x)];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(p0, i0);

  let pred = Pred::Reg(p0, Reg::Eax, Value(1));

  assert!(litmus(
    "ex4",
    &Prog(prog),
    State::new(&vec![p0]),
    &pred,
    PredType::Required,
  ));
}
