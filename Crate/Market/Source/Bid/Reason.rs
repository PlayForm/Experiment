//! Named reasons an allocation decision was made.
//!
//! Grounded in `rust/market-prototype/src/main.rs:95-159` (`caps must be
//! positive`, `empty bid identifiers`, `repeated bid ID`, `invalid information
//! gain`, `model_cohort_cap`, `mechanism_cap`, `insufficient_resources`,
//! `reserved_in_plan`), `MARKET.md:44-47` (caps) and `OPERATIONS.md:36`
//! (`duplicate_bid`, `budget_exceeded`). A reason is a typed value, not prose.

/// Why a bid was accepted or refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Reason {
    /// Capacity reserved in a deterministic plan (`main.rs:139`).
    ReservedInPlan,
    /// Per-model-family cap reached (`main.rs:132-133`).
    ModelCohortCap,
    /// Per-mechanism duplicate cap reached (`main.rs:134-135`).
    MechanismCap,
    /// Native resource vector did not fit (`main.rs:136-137`).
    InsufficientResources,
    /// Exact duplicate bid identifier (`main.rs:105-107`).
    DuplicateBid,
    /// Non-finite or negative information gain (`main.rs:108-110`).
    NonFiniteGain,
    /// A configured cap was not positive (`main.rs:97-99`).
    CapsMustBePositive,
    /// A required identifier was empty (`main.rs:102-104`).
    EmptyIdentifier,
}

impl Reason {
    /// All decision reasons in evaluation order.
    pub const ALL: [Self; 8] = [
        Self::ReservedInPlan,
        Self::ModelCohortCap,
        Self::MechanismCap,
        Self::InsufficientResources,
        Self::DuplicateBid,
        Self::NonFiniteGain,
        Self::CapsMustBePositive,
        Self::EmptyIdentifier,
    ];

    /// Stable machine label from the reference allocator.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReservedInPlan => "reserved_in_plan",
            Self::ModelCohortCap => "model_cohort_cap",
            Self::MechanismCap => "mechanism_cap",
            Self::InsufficientResources => "insufficient_resources",
            Self::DuplicateBid => "duplicate_bid",
            Self::NonFiniteGain => "invalid_information_gain",
            Self::CapsMustBePositive => "caps_must_be_positive",
            Self::EmptyIdentifier => "empty_bid_identifiers",
        }
    }

    /// Whether this reason is the sole acceptance reason.
    #[must_use]
    pub const fn is_acceptance(self) -> bool {
        matches!(self, Self::ReservedInPlan)
    }
}
