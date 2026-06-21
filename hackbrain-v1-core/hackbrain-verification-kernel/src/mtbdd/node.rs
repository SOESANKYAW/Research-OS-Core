use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum MTBDDNode {
    Terminal(f64),

    Decision {
        var: usize,
        low: Arc<MTBDDNode>,
        high: Arc<MTBDDNode>,
    },
}
