//! Coverage over required trusted sentences.
//! Grounded in `RESEARCH_GATE.md:91-93` (avoid leaking hidden oracle fixtures;
//! reveal only minimal violation categories) and `ARCHITECTURE.md:52-55`.

/// Reachability of the required trusted checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coverage {
    /// Symbols the oracle actually reached.
    pub reached: Vec<String>,
    /// Symbols that must be reached for an oracle pass.
    pub required: Vec<String>,
}

impl Coverage {
    /// Record reached and required symbols.
    #[must_use]
    pub fn of(reached: Vec<String>, required: Vec<String>) -> Self {
        Self { reached, required }
    }

    /// Whether every required sentence was reached.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.required
            .iter()
            .all(|symbol| self.reached.contains(symbol))
    }

    /// How many required sentences remain uncovered.
    #[must_use]
    pub fn missing(&self) -> usize {
        self.required
            .iter()
            .filter(|symbol| !self.reached.contains(symbol))
            .count()
    }
}
