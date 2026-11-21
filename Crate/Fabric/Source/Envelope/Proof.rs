//! Capability proofs and the rejected "usage seed as zero-knowledge" claim.
//!
//! `docs/01:49-53` rejects the terminology: two secrets are not a zero-
//! knowledge proof. `docs/04:31-50` requires proof of possession, short-lived
//! capabilities and trusted policy evaluation, and states a proprietary ZK
//! system is not needed. A claimed ZK proof is a typed rejected case.

/// One structurally recognized proof input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Proof {
    /// Proof of possession of the bound workload key.
    Possession,
    /// Verified workload identity from the issuer.
    Workload,
    /// Current policy version check.
    PolicyCurrent,
    /// Fresh, non-replayable nonce.
    NonceFresh,
    /// A claimed proprietary zero-knowledge proof.
    ClaimedZeroKnowledge,
}

impl Proof {
    /// Whether the proof is an accepted structural input.
    #[must_use]
    pub const fn accepted(&self) -> Verdict {
        match self {
            Self::Possession | Self::Workload | Self::PolicyCurrent | Self::NonceFresh => {
                Verdict::Allow
            }
            Self::ClaimedZeroKnowledge => Verdict::Deny,
        }
    }

    /// A claimed ZK proof is not accepted as authorization in this design
    /// (`docs/04:46-50`).
    #[must_use]
    pub const fn invented_system() -> bool {
        false
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Possession => "possession",
            Self::Workload => "workload",
            Self::PolicyCurrent => "policy-current",
            Self::NonceFresh => "nonce-fresh",
            Self::ClaimedZeroKnowledge => "claimed-zero-knowledge",
        }
    }
}
