extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Inst, Pred, PredType, Prog};
use x86_litmus::ast::{Proc, Operand, Reg, Value, MemLoc};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn iriw() {
  let x = Operand::MemLoc(MemLoc("x"));
  let y = Operand::MemLoc(MemLoc("y"));
  let eax = Operand::Reg(Reg::Eax);
  let ebx = Operand::Reg(Reg::Ebx);
  let ecx = Operand::Reg(Reg::Ecx);
  let edx = Operand::Reg(Reg::Edx);
  let one = Operand::Imm(Value(1));

  let p0 = Proc(0);
  let i0 = vec![Inst::Mov(x, one)];

  let p1 = Proc(1);
  let i1 = vec![Inst::Mov(y, one)];

  let p2 = Proc(2);
  let i2 = vec![Inst::Mov(eax, x), Inst::Mov(ebx, y)];

  let p3 = Proc(3);
  let i3 = vec![Inst::Mov(ecx, y), Inst::Mov(edx, x)];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(p0, i0);
  prog.insert(p1, i1);
  prog.insert(p2, i2);
  prog.insert(p3, i3);

  let pred = Pred::And(vec![
    Pred::Reg(p2, Reg::Eax, Value(1)),
    Pred::Reg(p2, Reg::Ebx, Value(0)),
    Pred::Reg(p3, Reg::Ecx, Value(1)),
    Pred::Reg(p3, Reg::Edx, Value(0)),
  ]);

  assert!(litmus(
    &Prog(prog),
    State::new(&vec![p0, p1, p2, p3]),
    &pred,
    PredType::Forbidden,
  ));
}
