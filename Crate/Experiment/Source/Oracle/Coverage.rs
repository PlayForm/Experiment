//! Oracle coverage and the test-gap report sent to a separate lane.
//!
//! Grounded in `AGENT_CONTRACTS.md:13` (Oracle reviewer: writes a test gap
//! report to a separate lane, forbidden from changing the oracle inline with
//! its evaluated candidate) and `ARCHITECTURE.md:54-55`. A gap cannot be
//! silently patched by the candidate under test: any oracle change is a
//! separate reviewable mutation.

/// How a coverage gap report relates to the oracle it inspects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Change {
    /// The report proposes a separate, reviewable mutation.
    Separate,
    /// The evaluated candidate attempted an inline oracle edit.
    Inline,
}

/// A gap in trusted oracle coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coverage {
    /// Oracle version this coverage describes.
    pub oracle_version: String,
    /// Number of trusted fixtures applied.
    pub fixtures: usize,
    /// Names of untrusted or missing behavior categories.
    pub gaps: Vec<String>,
    /// Whether the gap report author changed the oracle inline.
    pub change: Change,
}

impl Coverage {
    /// Whether the trusted coverage is sufficient for promotion. An empty
    /// fixture set or an inline change is never sufficient.
    #[must_use]
    pub fn is_sufficient(&self) -> bool {
        self.fixtures > 0 && self.gaps.is_empty() && matches!(self.change, Change::Separate)
    }

    /// Whether this report may itself modify the evaluated oracle. It may not;
    /// the change must be proposed as a separate mutation.
    #[must_use]
    pub const fn may_modify_oracle() -> bool {
        false
    }

    /// The lane this gap report is routed to, by name.
    #[must_use]
    pub const fn review_lane() -> &'static str {
        "oracle-review"
    }
}
