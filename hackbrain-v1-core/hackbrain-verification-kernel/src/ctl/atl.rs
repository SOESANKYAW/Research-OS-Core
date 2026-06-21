#[derive(Clone, Debug)]
pub enum ATLFormula {
    True,
    False,

    /// ⟨σ, π⟩ X φ
    Next(Box<ATLFormula>),

    /// ⟨σ, π⟩ G φ
    Globally(Box<ATLFormula>),

    /// ⟨σ, π⟩ F φ
    Finally(Box<ATLFormula>),

    /// φ ∧ ψ
    And(Box<ATLFormula>, Box<ATLFormula>),

    /// φ ∨ ψ
    Or(Box<ATLFormula>, Box<ATLFormula>),
}
