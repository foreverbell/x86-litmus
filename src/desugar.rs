use ast::{Proc, Operand, Reg};
use ast::{Inst, CoreInst};
use ast::{Prog, CoreProg};
use std::collections::BTreeMap;
use std::vec::Vec;

fn is_internal_register(operand: Operand) -> bool {
  match operand {
    Operand::Reg(Reg::Internal) => true,
    _ => false,
  }
}

fn desugar_helper(insts: &Vec<Inst>) -> Vec<CoreInst> {
  let mut desugared = vec![];

  for inst in insts.into_iter() {
    match *inst {
      Inst::Mov(operand1, operand2) |
      Inst::Xchg(operand1, operand2) => {
        if is_internal_register(operand1) || is_internal_register(operand2) {
          panic!("cannot use internal register");
        }
      },
      Inst::Mfence => (),
    }
  }

  for inst in insts.into_iter() {
    match *inst {
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
        match (operand1, operand2) {
          (Operand::Reg(reg), Operand::MemLoc(memloc)) |
          (Operand::MemLoc(memloc), Operand::Reg(reg)) => {
            desugared.push(CoreInst::Lock);
            desugared.push(CoreInst::Mov1(Reg::Internal, reg));
            desugared.push(CoreInst::Read(reg, memloc));
            desugared.push(CoreInst::Write1(memloc, Reg::Internal));
            desugared.push(CoreInst::Unlock);
          },
          (_, _) => panic!("unimplemented"),
        }
      },
      Inst::Mfence => {
        desugared.push(CoreInst::Mfence);
      },
    }
  }
  desugared
}

// Desugar Prog into CoreProg, also does some typechecking.
pub fn desugar(prog: &Prog) -> CoreProg {
  let mut desugared: BTreeMap<Proc, Vec<CoreInst>> = BTreeMap::new();

  for (processor, insts) in &prog.0 {
    desugared.insert(*processor, desugar_helper(insts));
  }
  CoreProg(desugared)
}
