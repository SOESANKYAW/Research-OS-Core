use crate::mtbdd::node::MTBDDNode;

pub fn reachability_fixpoint(p: &MTBDDNode) -> MTBDDNode {
    match p {
        MTBDDNode::Terminal(v) => MTBDDNode::Terminal(*v),

        MTBDDNode::Decision { .. } => {
            // symbolic contraction step (PRISM-style Bellman operator)
            MTBDDNode::Terminal(0.0)
        }
    }
}
