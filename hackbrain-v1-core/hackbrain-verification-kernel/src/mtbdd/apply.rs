use crate::mtbdd::{
    manager::{MTBDDManager, NodeId},
    cache::ApplyKey,
    ops::Op,
};

pub fn apply(
    mgr: &mut MTBDDManager,
    op: Op,
    a: NodeId,
    b: NodeId,
) -> NodeId {
    let key = ApplyKey {
        op: op_to_u8(&op),
        a: a.clone(),
        b: b.clone(),
    };

    if let Some(hit) = mgr.cache.get(&key) {
        return hit;
    }

    let na = mgr.get(&a).clone();
    let nb = mgr.get(&b).clone();

    // terminal case
    if na.value.is_some() && nb.value.is_some() {
        let v = crate::mtbdd::ops::apply_op(&op, na.value.unwrap(), nb.value.unwrap());
        let res = mgr.mk_terminal(v);
        mgr.cache.insert(key, res.clone());
        return res;
    }

    // recursive Shannon expansion
    let var = std::cmp::min(na.var, nb.var);

    let (a_low, a_high) = restrict(mgr, &a, var);
    let (b_low, b_high) = restrict(mgr, &b, var);

    let low = apply(mgr, op.clone(), a_low, b_low);
    let high = apply(mgr, op.clone(), a_high, b_high);

    let res = mgr.mk_node(var, low, high);
    mgr.cache.insert(key, res.clone());
    res
}

fn restrict(mgr: &mut MTBDDManager, node: &NodeId, var: usize) -> (NodeId, NodeId) {
    let n = mgr.get(node).clone();

    if n.var > var {
        return (node.clone(), node.clone());
    }

    if n.value.is_some() {
        return (node.clone(), node.clone());
    }

    // simplified structural propagation
    (n.low.clone(), n.high.clone())
}

fn op_to_u8(op: &Op) -> u8 {
    match op {
        Op::Add => 0,
        Op::Mul => 1,
        Op::Max => 2,
        Op::Min => 3,
    }
}
