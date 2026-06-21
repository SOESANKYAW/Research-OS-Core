use crate::{
    mtbdd::manager::{MTBDDManager, NodeId},
    mtbdd::apply::apply,
    mtbdd::ops::Op,
    bellman::semantics::{TransitionRelation, ValueFunction},
};

/// ===============================
/// SYMBOLIC BELLMAN SOLVER
/// ===============================

pub struct BellmanSolver {
    pub gamma: f64, // discount / stability factor
}

impl BellmanSolver {
    pub fn new(gamma: f64) -> Self {
        Self { gamma }
    }

    /// Main fixpoint iteration:
    /// V_{k+1} = Bellman(V_k)
    pub fn iterate(
        &self,
        mgr: &mut MTBDDManager,
        trans: &TransitionRelation,
        v: &ValueFunction,
        max_depth: usize,
    ) -> ValueFunction {
        let mut current = v.clone();

        for _ in 0..max_depth {
            let next = self.bellman_step(mgr, trans, &current);
            current = next;
        }

        current
    }

    /// ============================================
    /// SINGLE SYMBOLIC BELLMAN UPDATE STEP
    /// ============================================
    pub fn bellman_step(
        &self,
        mgr: &mut MTBDDManager,
        trans: &TransitionRelation,
        v: &ValueFunction,
    ) -> ValueFunction {

        // Step 1: BACKUP over transition relation
        let backed_up = self.expectation_over_transitions(
            mgr,
            trans.relation_root.clone(),
            v.root.clone(),
        );

        // Step 2: adversarial reduction (max-min game)
        let reduced = self.max_min_projection(mgr, backed_up);

        ValueFunction { root: reduced }
    }

    /// ============================================
    /// EXPECTATION OVER MTBDD TRANSITIONS
    /// P(s,a,b,s') * V(s')
    /// ============================================
    fn expectation_over_transitions(
        &self,
        mgr: &mut MTBDDManager,
        t: NodeId,
        v: NodeId,
    ) -> NodeId {

        // Multiply transition relation with value function
        // (symbolic composition in MTBDD space)

        apply(mgr, Op::Mul, t, v)
    }

    /// ============================================
    /// MAX-MIN OVER STRATEGY SPACES (ATL CORE)
    /// ============================================
    fn max_min_projection(
        &self,
        mgr: &mut MTBDDManager,
        node: NodeId,
    ) -> NodeId {

        // In PRISM terms:
        // - MAX over controller (sigma)
        // - MIN over adversary (pi)

        let maxed = apply(mgr, Op::Max, node.clone(), node.clone());
        let mined = apply(mgr, Op::Min, maxed, node);

        mined
    }
}
