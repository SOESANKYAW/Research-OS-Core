use crate::ctl::syntax::CTLFormula;
use crate::core::state::StateId;
use std::collections::HashSet;

pub struct ModelChecker;

impl ModelChecker {
    pub fn check(_phi: CTLFormula, initial: HashSet<StateId>) -> f64 {
        // FIXPOINT CORE PLACEHOLDER
        initial.len() as f64
    }
}
