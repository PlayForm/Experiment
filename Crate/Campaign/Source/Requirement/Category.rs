//! The nature of a requirement, separate from its section.
//!
//! A section says where a requirement lives; a category says what kind of
//! claim it makes. The distinction prevents a research question being read
//! as an enforced invariant, or a measured objective being read as a design
//! law (`REQUIREMENTS.md:43-103`, `:508-520`, `:1001-1032`).

/// The nature of a numbered requirement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Category {
    /// A non-negotiable design law.
    Invariant,
    /// A capability the product or laboratory must offer.
    Capability,
    /// A constraint on agents, measurement or promotion.
    Constraint,
    /// A tunable objective with declared weights and thresholds.
    Objective,
    /// An unresolved question requiring evidence, not an assertion.
    Research,
}

impl Category {
    /// Token for machine-readable status output.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Invariant => "invariant",
            Self::Capability => "capability",
            Self::Constraint => "constraint",
            Self::Objective => "objective",
            Self::Research => "research",
        }
    }

    /// Whether the category is enforced by deterministic code rather than
    /// evaluated by measurement or human review.
    #[must_use]
    pub const fn is_hard(self) -> bool {
        matches!(self, Self::Invariant | Self::Constraint)
    }
}
