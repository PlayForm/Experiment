//! The ten typed phase identifiers of the autonomous-week playbook.
//!
//! Grounded in `playbooks/autonomous-week.yaml:23-83` - baseline, scout,
//! mutate, static_gate, semantic_gate, measure, analyze, independent_confirm,
//! curate and integrate, in that exact order.

/// One phase of the playbook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Baseline,
    Scout,
    Mutate,
    Static,
    Semantic,
    Measure,
    Analyze,
    Confirm,
    Curate,
    Integrate,
}

impl Kind {
    /// All ten phases in playbook order.
    pub const ALL: [Self; 10] = [
        Self::Baseline,
        Self::Scout,
        Self::Mutate,
        Self::Static,
        Self::Semantic,
        Self::Measure,
        Self::Analyze,
        Self::Confirm,
        Self::Curate,
        Self::Integrate,
    ];

    /// The exact YAML `id` token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Scout => "scout",
            Self::Mutate => "mutate",
            Self::Static => "static_gate",
            Self::Semantic => "semantic_gate",
            Self::Measure => "measure",
            Self::Analyze => "analyze",
            Self::Confirm => "independent_confirm",
            Self::Curate => "curate",
            Self::Integrate => "integrate",
        }
    }
}
