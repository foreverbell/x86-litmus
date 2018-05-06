extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Inst, Pred, PredType, Prog};
use x86_litmus::ast::{Proc, Operand, Reg, Value, MemLoc};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn amd5() {
  let x = Operand::MemLoc(MemLoc("x"));
  let y = Operand::MemLoc(MemLoc("y"));
  let eax = Operand::Reg(Reg::Eax);
  let ebx = Operand::Reg(Reg::Ebx);
  let one = Operand::Imm(Value(1));

  let p0 = Proc(0);
  let i0 = vec![Inst::Mov(x, one), Inst::Mfence, Inst::Mov(eax, y)];

  let p1 = Proc(1);
  let i1 = vec![Inst::Mov(y, one), Inst::Mfence, Inst::Mov(ebx, x)];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(p0, i0);
  prog.insert(p1, i1);

  let pred = Pred::And(vec![
    Pred::Reg(p0, Reg::Eax, Value(0)),
    Pred::Reg(p1, Reg::Ebx, Value(0)),
  ]);

  assert!(litmus(
    "amd5",
    &Prog(prog),
    State::new(&vec![p0, p1]),
    &pred,
    PredType::Forbidden,
  ));
}
