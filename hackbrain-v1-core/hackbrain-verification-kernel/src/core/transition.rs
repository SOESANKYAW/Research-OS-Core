use crate::core::state::StateId;

#[derive(Clone, Debug)]
pub struct Transition {
    pub from: StateId,
    pub to: StateId,
    pub probability: f64,
}
