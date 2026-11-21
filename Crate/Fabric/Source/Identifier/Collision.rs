//! Fail-closed collision outcomes for critical identifiers.
//!
//! Identical tokens for different resources are a critical incident. The
//! system fails closed on namespace collisions and version conflicts, and it
//! also designs for semantic collisions where two namespaces canonicalize to
//! the same bytes (`docs/01:60-67`, `README.md:25-26`). Nothing waives a
//! collision for authorization, idempotency or commitments.

use crate::verdict::Verdict;

/// One collision finding from the private registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Collision {
    /// Two kinds claimed the same canonical bytes.
    Namespace,
    /// The same semantic identifier claimed two conflicting revisions.
    Version,
    /// Different namespaces canonicalized to the same bytes.
    Semantic,
    /// No collision was detected.
    Clear,
}

impl Collision {
    /// Every non-empty collision is fatal to admission.
    #[must_use]
    pub const fn refuses(&self) -> Verdict {
        match self {
            Self::Clear => Verdict::Allow,
            Self::Namespace | Self::Version | Self::Semantic => Verdict::Deny,
        }
    }

    /// A collision is logged to trusted audit, never sent to a worker.
    #[must_use]
    pub const fn operator_only(&self) -> bool {
        !matches!(self, Self::Clear)
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Namespace => "namespace",
            Self::Version => "version",
            Self::Semantic => "semantic",
            Self::Clear => "clear",
        }
    }
}
