//! The four-pass novelty filter and its versioned verdicts.
//!
//! Grounded in `RESEARCH_GATE.md:1-33` (pass 0 scope, pass 1 exact identity,
//! pass 2 canonical structural similarity, pass 3 semantic/hypothesis),
//! `ARCHITECTURE.md:75-80` (the six gate outcomes) and
//! `sql/v3_migration.sql:101-109` (the verdict CHECK constraint). Similarity is
//! advisory; only versioned policy approves or rejects.
//!
//! Exception declared: `Pass` is the typed form of the filter stage and has no
//! independent lifecycle outside `Novelty`.

/// The deterministic filter stage being applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pass {
    /// Scope, ancestor, feature family, contract digest and forbidden edits.
    Scope,
    /// Keyed source, normalized diff, recipe, binary and experiment key.
    ExactIdentity,
    /// Canonicalized syntax, changed symbols and control-flow tags.
    Structural,
    /// Mechanism taxonomy and optional versioned embedding similarity.
    Semantic,
}

/// The verdict emitted by the novelty gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Novelty {
    /// The exact experiment key was already measured.
    ExactCacheHit,
    /// A close prior experiment exists; advisory only.
    NearDuplicate,
    /// No prior exact or near match found.
    Novel,
    /// A mutually exclusive feature or scope conflict.
    Conflict,
    /// There is not enough evidence to decide.
    InsufficientProof,
    /// The patch failed the scope gate before any model or build time.
    Invalid,
}

impl Novelty {
    /// All six outcomes (`ARCHITECTURE.md:79-80`, `v3_migration.sql:103-108`).
    pub const ALL: [Self; 6] = [
        Self::ExactCacheHit,
        Self::NearDuplicate,
        Self::Novel,
        Self::Conflict,
        Self::InsufficientProof,
        Self::Invalid,
    ];

    /// The exact verdict token used in the ledger.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::ExactCacheHit => "EXACT_CACHE_HIT",
            Self::NearDuplicate => "NEAR_DUPLICATE",
            Self::Novel => "NOVEL",
            Self::Conflict => "CONFLICT",
            Self::InsufficientProof => "INSUFFICIENT_PROOF",
            Self::Invalid => "INVALID",
        }
    }

    /// Whether this verdict lets the candidate reach the builder.
    #[must_use]
    pub const fn buildable(self) -> bool {
        matches!(self, Self::Novel | Self::ExactCacheHit)
    }

    /// Whether the decision came from proof rather than an advisory signal.
    #[must_use]
    pub const fn is_proven(self) -> bool {
        matches!(self, Self::ExactCacheHit | Self::Invalid | Self::Conflict)
    }
}
