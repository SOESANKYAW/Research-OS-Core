use crate::core::state::SymbolicState;

pub struct QuotientClass {
    pub representative: SymbolicState,
    pub members: Vec<SymbolicState>,
}
