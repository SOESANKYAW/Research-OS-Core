use crate::dbm::constraint::DBMConstraint;

#[derive(Clone, Debug)]
pub struct Zone {
    pub n_clocks: usize,
    pub constraints: Vec<DBMConstraint>,
}
