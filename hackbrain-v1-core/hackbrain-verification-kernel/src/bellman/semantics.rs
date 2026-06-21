use crate::mtbdd::manager::{MTBDDManager, NodeId};
use crate::mtbdd::ops::Op;

/// Encoded transition relation:
/// T(s, a, b, s') stored implicitly in MTBDD form
pub struct TransitionRelation {
    pub relation_root: NodeId,
}

/// Strategy spaces (abstract ATL control)
pub struct StrategySpace {
    pub max_actions: Vec<usize>,
    pub min_actions: Vec<usize>,
}

/// Value function over states (MTBDD)
#[derive(Clone)]
pub struct ValueFunction {
    pub root: NodeId,
}
