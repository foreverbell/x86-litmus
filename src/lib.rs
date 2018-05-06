pub mod ast;
pub mod state;
mod desugar;
mod run;

use ast::{Prog, Pred};
use desugar::desugar;
use run::run;
use state::State;

pub fn litmus(prog: &Prog, init: State, pred: &Pred) -> bool {
  let desugared = desugar(prog);
  let terminals = run(desugared, init);

  for terminal in &terminals {
    if !terminal.satisfy(pred) {
      return false;
    }
  }
  true
}
