//! The two declared purposes of a mutation-parent edge.

/// Why one mutation descends from another.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Purpose {
    /// Direct causal descent.
    Causal,
    /// Merge or integration ancestry.
    Merge,
}

impl Purpose {
    /// The declared value.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Causal => "causal",
            Self::Merge => "merge",
        }
    }
}
