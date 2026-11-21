//! The four-pass gate decision, with correlated failure reasons retained.
//! Grounded in `ARCHITECTURE.md:75-80` (the six gate outcomes) and
//! `RESEARCH_GATE.md:3-33` (fast-first filtering; only versioned policy
//! approves or rejects; ambiguity is kept, never silently erased).

/// The outcome of the novelty and scope gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Decision {
    /// Identical executable, environment, fixture and protocol.
    ExactCacheHit,
    /// Structurally or semantically near a known experiment.
    NearDuplicate,
    /// Admissible and new.
    Novel,
    /// Conflicting scope or feature interaction.
    Conflict,
    /// Proof or policy version is insufficient.
    InsufficientProof,
    /// Off-scope, malformed or otherwise invalid.
    Invalid,
}

impl Decision {
    /// Only a novel candidate proceeds to build and oracle.
    #[must_use]
    pub const fn allows_build(self) -> bool {
        matches!(self, Self::Novel)
    }

    /// Whether the outcome must remain discoverable as negative evidence.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Conflict | Self::InsufficientProof | Self::Invalid)
    }

    /// Stable key for the evidence ledger.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::ExactCacheHit => "exact_cache_hit",
            Self::NearDuplicate => "near_duplicate",
            Self::Novel => "novel",
            Self::Conflict => "conflict",
            Self::InsufficientProof => "insufficient_proof",
            Self::Invalid => "invalid",
        }
    }
}
