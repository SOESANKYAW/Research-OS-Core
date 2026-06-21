use crate::fixpoint::lattice::Mtbdd;
use crate::fixpoint::delta::infinity_norm_delta;

#[derive(Debug, Clone)]
pub struct ConvergenceConfig {
    pub epsilon: f64,
    pub max_iters: usize,
}

pub enum ConvergenceResult {
    Converged { iterations: usize, value: Mtbdd },
    Diverged { last: Mtbdd },
}

pub fn has_converged(prev: &Mtbdd, next: &Mtbdd, eps: f64) -> bool {
    let delta = infinity_norm_delta(prev, next);
    delta < eps
}
