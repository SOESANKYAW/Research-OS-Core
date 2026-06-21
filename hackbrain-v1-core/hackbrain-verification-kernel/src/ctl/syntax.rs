#[derive(Clone, Debug)]
pub enum CTLFormula {
    True,
    False,
    Atomic(String),

    Not(Box<CTLFormula>),
    And(Box<CTLFormula>, Box<CTLFormula>),

    EX(Box<CTLFormula>),
    EG(Box<CTLFormula>),
    EU(Box<CTLFormula>, Box<CTLFormula>),
}
