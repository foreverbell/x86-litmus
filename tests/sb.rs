extern crate x86_litmus;

use std::collections::BTreeMap;
use x86_litmus::ast::{Proc, Inst, Operand, Reg, Value, MemLoc, Pred, Prog};
use x86_litmus::litmus;
use x86_litmus::state::State;

#[test]
fn sb() {
  let proc0 = Proc(0);
  let prog0 =
    vec![
      Inst::Mov(Operand::MemLoc(MemLoc("x")), Operand::Imm(Value(1))),
      Inst::Mov(Operand::MemLoc(MemLoc("y")), Operand::Imm(Value(1))),
    ];

  let proc1 = Proc(1);
  let prog1 =
    vec![
      Inst::Mov(Operand::Reg(Reg::Eax), Operand::MemLoc(MemLoc("y"))),
      Inst::Mov(Operand::Reg(Reg::Ebx), Operand::MemLoc(MemLoc("x"))),
    ];

  let mut prog: BTreeMap<Proc, Vec<Inst>> = BTreeMap::new();
  prog.insert(proc0, prog0);
  prog.insert(proc1, prog1);

  let pred = Pred::Not(Box::new(Pred::And(vec![
    Pred::Reg(proc1, Reg::Eax, Value(1)),
    Pred::Reg(proc1, Reg::Ebx, Value(0)),
  ])));

  assert!(litmus(&Prog(prog), State::new(&vec![proc0, proc1]), &pred));
}
