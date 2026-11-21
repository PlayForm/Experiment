//! A policy decision with a withheld, audit-only reason.
//!
//! Mirrors the `allow/deny + policy digest` output of the policy engine row in
//! `docs/08:9` and the rule that policy-denial reasons never reach a worker
//! (`docs/07:28-30`). `Deferred` is the honest outcome when a trusted service
//! is unreachable; the decision fails closed.

use crate::verdict::Verdict;

/// One structural policy outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Structural rules matched; a capability may still be required.
    Allow,
    /// At least one structural rule failed.
    Deny,
    /// A trusted input was unavailable; the safe default is to deny.
    Deferred,
}

impl Decision {
    /// Lift a structural verdict into a policy decision.
    #[must_use]
    pub const fn from_verdict(verdict: Verdict) -> Self {
        match verdict {
            Verdict::Allow => Self::Allow,
            Verdict::Deny => Self::Deny,
        }
    }

    /// Conjoin two decisions; a single failure denies the whole decision.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Allow, Self::Allow) => Self::Allow,
            (Self::Deferred, _) | (_, Self::Deferred) => Self::Deferred,
            _ => Self::Deny,
        }
    }

    /// Whether the decision authorizes any operation.
    #[must_use]
    pub const fn grants(self) -> bool {
        matches!(self, Self::Allow)
    }

    /// The only value a worker may observe, regardless of the decision.
    #[must_use]
    pub const fn worker_status(self) -> &'static str {
        crate::NEUTRAL_STATUS
    }
}
