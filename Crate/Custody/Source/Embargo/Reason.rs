//! Trusted-only cause codes for denied embargo decisions.
//!
//! These values exist so an operator sees the real reason and incident status.
//! No worker-visible type in this crate carries a [`Reason`]; the only worker
//! projection is the fixed neutral receipt
//! (`docs/17:9`, `docs/20:57-70`, `docs/22:47-69`).

/// A cause retained in the encrypted audit ledger.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reason {
    /// Subject is under embargo for this recipient or track.
    Embargo,
    /// A trade graph edge denied the peer relationship.
    Trade,
    /// License or EULA terms did not permit the use.
    License,
    /// Jurisdiction-sensitive terms need owner review.
    Jurisdiction,
    /// Sealed holdout or cohort data was requested.
    Holdout,
    /// Conflict of interest or anti-collusion policy fired.
    Conflict,
    /// No trusted reason was available; fail closed.
    Unknown,
}

impl Reason {
    /// The operator label; never emitted to a worker.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Embargo => "embargo",
            Self::Trade => "trade",
            Self::License => "license",
            Self::Jurisdiction => "jurisdiction",
            Self::Holdout => "holdout",
            Self::Conflict => "conflict-of-interest",
            Self::Unknown => "unknown",
        }
    }

    /// A reason code is always audit-only.
    #[must_use]
    pub fn worker_visible(&self) -> bool {
        match self {
            Self::Embargo
            | Self::Trade
            | Self::License
            | Self::Jurisdiction
            | Self::Holdout
            | Self::Conflict
            | Self::Unknown => false,
        }
    }
}
