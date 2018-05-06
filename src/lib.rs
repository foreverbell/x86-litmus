pub mod ast;
pub mod state;
mod desugar;
mod run;

use ast::{Prog, Pred, PredType};
use desugar::desugar;
use run::run;
use state::State;

pub fn litmus(
  prog: &Prog,
  init: State,
  pred: &Pred,
  pred_type: PredType,
) -> bool {
  let desugared = desugar(prog);
  let terminals = run(desugared, init);

  match pred_type {
    PredType::Forbidden => {
      for terminal in &terminals {
        if terminal.satisfy(pred) {
          return false;
        }
      }
      return true;
    },
    PredType::Required => {
      for terminal in &terminals {
        if !terminal.satisfy(pred) {
          return false;
        }
      }
      return true;
    },
    PredType::Allowed => {
      for terminal in &terminals {
        if terminal.satisfy(pred) {
          return true;
        }
      }
      return false;
    },
  }
}
