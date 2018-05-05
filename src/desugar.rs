use ast::Proc;
use ast::{SurfaceInstr, CoreInstr};
use ast::{SurfaceProg, CoreProg};
use std::collections::HashMap;
use std::vec::Vec;

fn desugar_helper(instrs: &Vec<SurfaceInstr>) -> Vec<CoreInstr> {
  let mut desugared = vec![];

  for instr in instrs.into_iter() {
    match *instr {
      SurfaceInstr::MOV(dst, src) => {
        desugared.push(CoreInstr::MOV(dst, src));
      }
      SurfaceInstr::XCHG(dst1, dst2) => {
        desugared.push(CoreInstr::LOCK);
        desugared.push(CoreInstr::XCHG(dst1, dst2));
        desugared.push(CoreInstr::UNLOCK);
      }
      SurfaceInstr::MFENCE => { 
        desugared.push(CoreInstr::MFENCE);
      }
    }

  }
  desugared
}

pub fn desugar(prog: &SurfaceProg) -> CoreProg {
  let mut desugared: HashMap<Proc, Vec<CoreInstr>> = HashMap::new();

  for (processor, instrs) in &prog.0 {
    desugared.insert(*processor, desugar_helper(instrs)).unwrap();
  }
  CoreProg(desugared)
}
