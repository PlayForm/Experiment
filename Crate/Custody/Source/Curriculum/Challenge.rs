//! The eight challenge families from `docs/27:22-30`.
//!
//! The model receives only a generic Rust source task and permitted references,
//! never the curriculum catalog or the institutional arrangement.

/// One preregistered challenge family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Challenge {
    /// `exploration`: identify unexplained bottlenecks.
    Explore,
    /// `implementation`: implement one hypothesis.
    Implement,
    /// `skepticism`: propose failure modes.
    Skepticism,
    /// `test-design`: propose edge cases.
    Tests,
    /// `replication`: independently test a claim.
    Replicate,
    /// `hardware-specialization`: optimize a known target cohort.
    Specialize,
    /// `cross-repository-transfer`: propose, do not claim, transferred benefit.
    Transfer,
    /// `publication`: propose a private research narrative.
    Publish,
}

impl Challenge {
    /// All families in the order the curriculum lists them.
    #[must_use]
    pub fn all() -> [Self; 8] {
        [
            Self::Explore,
            Self::Implement,
            Self::Skepticism,
            Self::Tests,
            Self::Replicate,
            Self::Specialize,
            Self::Transfer,
            Self::Publish,
        ]
    }

    /// The exact family spelling from the curriculum record.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Explore => "exploration",
            Self::Implement => "implementation",
            Self::Skepticism => "skepticism",
            Self::Tests => "test-design",
            Self::Replicate => "replication",
            Self::Specialize => "hardware-specialization",
            Self::Transfer => "cross-repository-transfer",
            Self::Publish => "publication",
        }
    }

    /// Whether a model may assign this challenge to itself.
    #[must_use]
    pub fn model_selected(&self) -> bool {
        match self {
            Self::Explore
            | Self::Implement
            | Self::Skepticism
            | Self::Tests
            | Self::Replicate
            | Self::Specialize
            | Self::Transfer
            | Self::Publish => false,
        }
    }
}
