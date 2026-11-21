//! Advisory semantic and structural proximity; never proof of equivalence.
//!
//! Grounded in `RESEARCH_GATE.md:17-49` (canonical structural similarity,
//! probabilistic hint, mechanism taxonomy nearest neighbour, feature
//! compatibility classes, same-symbol touches as interaction risk) and
//! `REQUIREMENTS_V3.md` NOV-003/NOV-004/NOV-005. An advisory classifier may
//! categorize an ambiguous overlap but cannot erase a novel patch or promote
//! evidence.
//!
//! Exception declared: `Source` is the typed form of this one field and has no
//! independent lifecycle outside `Proximity`.

/// Compatibility of two mechanism or feature identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Compatibility {
    /// The features can coexist in one binary.
    Independent,
    /// The features touch overlapping code but may coexist.
    Overlapping,
    /// Two strategies for one exclusive stage cannot both be active.
    MutuallyExclusive,
    /// The features may interact and require a fresh test.
    Interacting,
    /// Compatibility cannot be established.
    Unknown,
}

/// Who produced the proximity score.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    /// A deterministic canonical index or token comparator.
    Deterministic,
    /// An advisory model categorizing an ambiguous overlap.
    Model,
}

/// An advisory proximity report for one candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proximity {
    /// Nearest known experiment ids with sanitized summaries.
    pub neighbors: Vec<String>,
    /// Compatibility of the candidate's mechanism family.
    pub compatibility: Compatibility,
    /// Canonicalized syntax similarity in the range 0.0..=1.0.
    pub structural_score: u32,
    /// Version of the taxonomy or embedding used, for provenance.
    pub taxonomy_version: String,
    /// Whether an advisory model produced the score.
    pub source: Source,
}

impl Proximity {
    /// Whether this report can prove semantic equivalence. It never can
    /// (`RESEARCH_GATE.md:22-23`).
    #[must_use]
    pub const fn is_proof() -> bool {
        false
    }

    /// Whether the candidate must be freshly tested despite a near match.
    #[must_use]
    pub const fn requires_fresh_test(&self) -> bool {
        matches!(
            self.compatibility,
            Compatibility::Interacting
                | Compatibility::MutuallyExclusive
                | Compatibility::Unknown
        )
    }

    /// The compatibility token recorded in the gate verdict.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        match self.compatibility {
            Compatibility::Independent => "independent",
            Compatibility::Overlapping => "overlapping",
            Compatibility::MutuallyExclusive => "mutually_exclusive",
            Compatibility::Interacting => "interacting",
            Compatibility::Unknown => "unknown",
        }
    }
}
