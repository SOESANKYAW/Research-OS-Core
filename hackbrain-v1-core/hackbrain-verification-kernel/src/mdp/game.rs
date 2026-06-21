use std::collections::HashMap;
use crate::core::state::StateId;

/// Player roles in Q-ZB-CTSG
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    Scheduler,   // σ (maximizer)
    Adversary,   // π (minimizer)
}

/// Game transition kernel:
/// P(s' | s, action_scheduler, action_adversary)
pub type Prob = f64;

#[derive(Clone, Debug)]
pub struct GameTransition {
    pub from: StateId,
    pub to: StateId,
    pub probability: Prob,
    pub sched_action: usize,
    pub adv_action: usize,
}

/// Symbolic transition relation (compressed pre-MTBDD layer)
#[derive(Clone, Debug)]
pub struct GameKernel {
    pub transitions: Vec<GameTransition>,

    // action spaces
    pub scheduler_actions: usize,
    pub adversary_actions: usize,
}

impl GameKernel {
    pub fn new(s_actions: usize, a_actions: usize) -> Self {
        Self {
            transitions: vec![],
            scheduler_actions: s_actions,
            adversary_actions: a_actions,
        }
    }

    pub fn add_transition(&mut self, t: GameTransition) {
        self.transitions.push(t);
    }
}
