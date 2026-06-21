use crate::ctl::syntax::CTLFormula;
use crate::core::state::StateId;
use std::collections::HashSet;

pub fn eval(_phi: CTLFormula, states: HashSet<StateId>) -> HashSet<StateId> {
    states // placeholder symbolic hook
}
