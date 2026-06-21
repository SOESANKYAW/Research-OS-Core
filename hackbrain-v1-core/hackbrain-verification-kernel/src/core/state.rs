#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StateId(pub u64);

#[derive(Clone, Debug)]
pub struct SymbolicState {
    pub id: StateId,
    pub valuation: Vec<f64>, // lifted abstraction from DBM zone
}
