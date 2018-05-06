use ast::{Proc, Operand};
use ast::{Inst, CoreInst};
use ast::{Prog, CoreProg};
use std::collections::BTreeMap;
use std::vec::Vec;

fn desugar_helper(insts: &Vec<Inst>) -> Vec<CoreInst> {
  let mut desugared = vec![];

  for instr in insts.into_iter() {
    match *instr {
      Inst::Mov(operand1, operand2) => {
        match (operand1, operand2) {
          (Operand::Imm(_), _) => {
            panic!("dest operand cannot be immediate value");
          },
          (Operand::MemLoc(_), Operand::MemLoc(_)) => {
            panic!("cannot move between memory locations");
          },
          (Operand::Reg(reg1), Operand::Reg(reg2)) => {
            desugared.push(CoreInst::Mov1(reg1, reg2));
          },
          (Operand::Reg(reg), Operand::Imm(imm)) => {
            desugared.push(CoreInst::Mov2(reg, imm));
          },
          (Operand::Reg(reg), Operand::MemLoc(memloc)) => {
            desugared.push(CoreInst::Read(reg, memloc));
          },
          (Operand::MemLoc(memloc), Operand::Reg(reg)) => {
            desugared.push(CoreInst::Write1(memloc, reg));
          },
          (Operand::MemLoc(memloc), Operand::Imm(imm)) => {
            desugared.push(CoreInst::Write2(memloc, imm));
          },
        }
      },
      Inst::Xchg(operand1, operand2) => {
        desugared.push(CoreInst::Lock);
        match (operand1, operand2) {
          (Operand::Reg(reg), Operand::MemLoc(memloc)) => {
            desugared.push(CoreInst::Xchg(reg, memloc));
          },
          (Operand::MemLoc(memloc), Operand::Reg(reg)) => {
            desugared.push(CoreInst::Xchg(reg, memloc));
          },
          (_, _) => panic!("unimplemented"),
        }
        desugared.push(CoreInst::Unlock);
      },
      Inst::Mfence => {
        desugared.push(CoreInst::Mfence);
      },
    }
  }
  desugared
}

pub fn desugar(prog: &Prog) -> CoreProg {
  let mut desugared: BTreeMap<Proc, Vec<CoreInst>> = BTreeMap::new();

  for (processor, insts) in &prog.0 {
    desugared.insert(*processor, desugar_helper(insts));
  }
  CoreProg(desugared)
}
