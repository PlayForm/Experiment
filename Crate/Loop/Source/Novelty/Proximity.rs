//! Feature-set compatibility and structural similarity as hints only.
//! Grounded in `RESEARCH_GATE.md:34-49` (capability, strategy and mechanism
//! are three separate identities; comparator is advisory and never proves
//! semantic equivalence) and `:25-32` (advisory classifier cannot erase a
//! novel patch or promote evidence).

/// A compatibility relation between two candidates or features.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Proximity {
    /// Independent mechanisms that may coexist.
    Independent,
    /// Overlapping but not exclusive.
    Overlapping,
    /// Two strategies for one exclusive execution stage.
    MutuallyExclusive,
    /// Interacting changes that need a combined measurement.
    Interacting,
    /// Comparator could not decide; never treated as a duplicate.
    Unknown,
}

impl Proximity {
    /// A structural hint is never an exact-equivalence proof.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        false
    }

    /// Whether the relation permits two features in one binary.
    #[must_use]
    pub const fn may_coexist(self) -> bool {
        matches!(self, Self::Independent | Self::Overlapping)
    }

    /// Stable key for the gate decision record.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Independent => "independent",
            Self::Overlapping => "overlapping",
            Self::MutuallyExclusive => "mutually_exclusive",
            Self::Interacting => "interacting",
            Self::Unknown => "unknown",
        }
    }
}
