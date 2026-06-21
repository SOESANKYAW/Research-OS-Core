use std::sync::Arc;
use crate::core::state::StateId;

/// MTBDD node (binary decision diagram over state + action space)
#[derive(Clone, Debug)]
pub enum BddNode {
    Terminal(f64), // value function leaf

    Decision {
        state_var: usize,
        low: Arc<BddNode>,
        high: Arc<BddNode>,
    },
}

/// Policy representation in symbolic form:
/// π: State → Distribution(Action)
#[derive(Clone, Debug)]
pub struct PolicyBDD {
    pub root: Arc<BddNode>,
}

impl PolicyBDD {
    pub fn evaluate(&self, state: StateId) -> f64 {
        Self::eval_node(&self.root, state.0 as usize)
    }

    fn eval_node(node: &BddNode, state: usize) -> f64 {
        match node {
            BddNode::Terminal(v) => *v,

            BddNode::Decision { state_var, low, high } => {
                if state & (1 << state_var) == 0 {
                    Self::eval_node(low, state)
                } else {
                    Self::eval_node(high, state)
                }
            }
        }
    }
}
