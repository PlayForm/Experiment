//! The eight independent key authorities.
//!
//! Mirrors `docs/04:1-24`. Each authority has a distinct domain-separation
//! label; HKDF may derive subkeys from one securely held key for an approved
//! scope but cannot create independent entropy (`docs/01:43-46`,
//! `docs/04:25-28`). No authority can sign another authority's artifact
//! (`docs/04:3`).

use crate::verdict::Verdict;

/// One logical key authority in the fabric.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Hierarchy {
    /// Rarely online; signs admission and recovery policy.
    GovernanceRoot,
    /// Short-lived workload identities for trusted Rust services.
    WorkloadIssuer,
    /// Signs canonical envelopes and evidence attestations.
    MessageSigner,
    /// Per-recipient, per-epoch HPKE recipient key.
    HPKERecipient,
    /// Randomly generated per artifact or tenant.
    DataEncryption,
    /// Independent keyed-MAC material for one view and epoch.
    Pseudonymization,
    /// Durable event-commitment signing key.
    AuditSigner,
    /// Independent restoration path with approval quorum.
    Recovery,
}

impl Hierarchy {
    /// The stable domain-separation label for this authority.
    #[must_use]
    pub const fn domain(&self) -> &'static str {
        match self {
            Self::GovernanceRoot => "v62/root/governance",
            Self::WorkloadIssuer => "v62/issuer/workload",
            Self::MessageSigner => "v62/sign/message",
            Self::HPKERecipient => "v62/hpke/recipient",
            Self::DataEncryption => "v62/dek/artifact",
            Self::Pseudonymization => "v62/mac/pseudonym",
            Self::AuditSigner => "v62/sign/audit",
            Self::Recovery => "v62/recovery/quorum",
        }
    }

    /// All eight authorities, in the order of `docs/04:1-24`.
    #[must_use]
    pub const fn all() -> [Self; 8] {
        [
            Self::GovernanceRoot,
            Self::WorkloadIssuer,
            Self::MessageSigner,
            Self::HPKERecipient,
            Self::DataEncryption,
            Self::Pseudonymization,
            Self::AuditSigner,
            Self::Recovery,
        ]
    }

    /// Two different authorities are separate even when their handles look
    /// alike; a pseudonymization key is never reusable as an encryption key.
    #[must_use]
    pub fn separate_from(&self, other: &Self) -> Verdict {
        Verdict::of(self != other)
    }

    /// Whether this authority is rotated on its own schedule rather than
    /// derived from one compromised parent secret (`README.md:30-32`).
    #[must_use]
    pub const fn independently_rotated(&self) -> bool {
        true
    }
}
