use crate::fixpoint::lattice::Mtbdd;
use crate::fixpoint::convergence::{ConvergenceConfig, ConvergenceResult, has_converged};

pub struct BellmanOps;

impl BellmanOps {
    pub fn bellman(&self, v: &Mtbdd) -> Mtbdd {
        v.clone() // placeholder for bellman operation
    }
}

pub struct FixpointEngine {
    pub config: ConvergenceConfig,
    pub ops: BellmanOps,
}

impl FixpointEngine {
    pub fn run(&self, init: Mtbdd) -> ConvergenceResult {
        let mut v_prev = init.clone();
        let mut v_curr = v_prev.clone();

        for i in 0..self.config.max_iters {

            // 🔷 symbolic Bellman update over MTBDD
            v_curr = self.ops.bellman(&v_prev);

            // 🔷 structural + numeric convergence
            if has_converged(&v_prev, &v_curr, self.config.epsilon) {
                return ConvergenceResult::Converged {
                    iterations: i,
                    value: v_curr,
                };
            }

            v_prev = v_curr.clone();
        }

        ConvergenceResult::Diverged { last: v_curr }
    }
}

pub fn extract_ass_v3(result: &Mtbdd, initial_state_id: usize) -> f64 {
    result.evaluate_at(initial_state_id)
}
