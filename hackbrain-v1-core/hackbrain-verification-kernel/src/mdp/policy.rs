use crate::mtbdd::policy_bdd::PolicyBDD;
use crate::core::state::StateId;

/// Strategy pair = (Scheduler, Adversary)
pub struct StrategyProfile {
    pub scheduler: PolicyBDD,
    pub adversary: PolicyBDD,
}

impl StrategyProfile {
    pub fn sched_action(&self, s: StateId) -> f64 {
        self.scheduler.evaluate(s.clone())
    }

    pub fn adv_action(&self, s: StateId) -> f64 {
        self.adversary.evaluate(s)
    }
}
