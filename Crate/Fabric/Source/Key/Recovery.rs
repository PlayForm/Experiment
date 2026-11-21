//! Independent recovery paths for key material.
//!
//! `docs/04:8` specifies approval quorum and offline escrow; `docs/01:46-48`
//! states recovery material cannot be deduced solely from public genesis
//! metadata; `README.md:32-33` requires independent recovery materials. No key
//! is restored here.

use crate::verdict::Verdict;

/// One declared key-recovery path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Recovery {
    /// Restored from independently escrowed material.
    Escrowed,
    /// Restored after a recorded approval quorum.
    QuorumApproved,
    /// Illegitimately claimed from the public genesis commitment.
    DeducedFromGenesis,
    /// Escalated to audited break-glass human review.
    Escalated,
}

impl Recovery {
    /// Whether the path is an authorized recovery authority.
    #[must_use]
    pub const fn allowed(&self) -> Verdict {
        match self {
            Self::Escrowed | Self::QuorumApproved | Self::Escalated => Verdict::Allow,
            Self::DeducedFromGenesis => Verdict::Deny,
        }
    }

    /// Whether the path is independent of the online root
    /// (`docs/01:43-45`).
    #[must_use]
    pub const fn independent(&self) -> bool {
        !matches!(self, Self::DeducedFromGenesis)
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Escrowed => "escrowed",
            Self::QuorumApproved => "quorum-approved",
            Self::DeducedFromGenesis => "deduced-from-genesis",
            Self::Escalated => "escalated",
        }
    }
}
