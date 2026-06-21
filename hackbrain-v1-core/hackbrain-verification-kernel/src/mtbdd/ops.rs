#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Max,
    Min,
}

pub fn apply_op(op: &Op, x: f64, y: f64) -> f64 {
    match op {
        Op::Add => x + y,
        Op::Mul => x * y,
        Op::Max => x.max(y),
        Op::Min => x.min(y),
    }
}
