//! The links of the minimal lineage proof chain.
//!
//! Grounded in `REQUIREMENTS.md:752-772`: Campaign → Hypothesis → Mutation →
//! Source → Build → Artifact → Oracle → Experiment → Trial → Analysis →
//! Decision → Next. Each causal lane is linear; forks and joins are recorded
//! separately in `Ancestry`. A benchmark result never becomes the parent of
//! source code.

/// One node kind in the proof chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Link {
    Campaign,
    Hypothesis,
    Mutation,
    Source,
    Build,
    Artifact,
    Oracle,
    Experiment,
    Trial,
    Analysis,
    Decision,
    Next,
}

impl Link {
    /// All links in proof-chain order.
    pub const ALL: [Self; 12] = [
        Self::Campaign,
        Self::Hypothesis,
        Self::Mutation,
        Self::Source,
        Self::Build,
        Self::Artifact,
        Self::Oracle,
        Self::Experiment,
        Self::Trial,
        Self::Analysis,
        Self::Decision,
        Self::Next,
    ];

    /// The label used in the proof-chain diagram.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Campaign => "Campaign C",
            Self::Hypothesis => "Hypothesis H",
            Self::Mutation => "Mutation M",
            Self::Source => "Source S",
            Self::Build => "Build B",
            Self::Artifact => "Artifact A",
            Self::Oracle => "Oracle O",
            Self::Experiment => "Experiment E",
            Self::Trial => "Trial T",
            Self::Analysis => "Analysis X",
            Self::Decision => "Decision D",
            Self::Next => "Next mutation or integration request",
        }
    }
}
