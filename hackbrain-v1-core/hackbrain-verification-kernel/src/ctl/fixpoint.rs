use crate::mdp::game::{GameKernel, Player};
use crate::core::state::StateId;

/// Value function over quotient states
pub type ValueFn = Vec<f64>;

pub struct FixpointEngine;

impl FixpointEngine {

    /// MINIMAX Bellman operator:
    /// V(s) = sup_σ inf_π E[V(s')]
    pub fn minimax_step(
        kernel: &GameKernel,
        v: &ValueFn,
    ) -> ValueFn {
        let mut new_v = vec![0.0; v.len()];

        for s in 0..v.len() {
            let mut best_sched = f64::NEG_INFINITY;

            // Scheduler maximization
            for sigma in 0..kernel.scheduler_actions {

                let mut worst_adv = f64::INFINITY;

                // Adversary minimization
                for pi in 0..kernel.adversary_actions {

                    let mut expectation = 0.0;

                    for t in &kernel.transitions {
                        if t.from.0 as usize == s &&
                           t.sched_action == sigma &&
                           t.adv_action == pi {

                            expectation += t.probability * v[t.to.0 as usize];
                        }
                    }

                    worst_adv = worst_adv.min(expectation);
                }

                best_sched = best_sched.max(worst_adv);
            }

            new_v[s] = best_sched;
        }

        new_v
    }

    /// Fixpoint convergence
    pub fn solve(
        kernel: &GameKernel,
        mut v: ValueFn,
        epsilon: f64,
    ) -> ValueFn {
        loop {
            let next = Self::minimax_step(kernel, &v);

            let delta = v.iter()
                .zip(next.iter())
                .map(|(a,b)| (a-b).abs())
                .fold(0.0, f64::max);

            v = next;

            if delta < epsilon {
                break;
            }
        }

        v
    }
}
