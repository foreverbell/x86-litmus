use ast::{SurfaceInstr, CoreInstr};
use std::vec::Vec;

fn desugar(in_instrs: &Vec<SurfaceInstr>) -> Vec<CoreInstr> {
  let mut out_instrs = vec![];

  for instr in in_instrs.into_iter() {
    match *instr {
      SurfaceInstr::MOV(dst, src) => {
        out_instrs.push(CoreInstr::MOV(dst, src));
      }
      SurfaceInstr::XCHG(dst1, dst2) => {
        out_instrs.push(CoreInstr::LOCK);
        out_instrs.push(CoreInstr::XCHG(dst1, dst2));
        out_instrs.push(CoreInstr::UNLOCK);
      }
      SurfaceInstr::MFENCE => { 
        out_instrs.push(CoreInstr::MFENCE);
      }
    }

  }
  out_instrs
}
