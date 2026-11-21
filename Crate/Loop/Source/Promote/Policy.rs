//! The signed release policy: deny overrides allow, unknown fails closed.
//! Grounded in `26-acceptance-and-build-plan.md:11-13` (deny overrides allow;
//! any unknown fails closed) and `17-v61-constitution.md:60-63` (rights that
//! cannot be mechanically resolved fail closed pending human review).

use experiment::evaluate::verdict::Verdict;

/// A release decision from the fixed, signed policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Policy {
    /// All preconditions satisfied and no denial present.
    Allow,
    /// A denial, unknown condition or missing approval.
    Deny,
}

impl Policy {
    /// Apply the policy to a trusted verdict; a non-promotable verdict denies.
    #[must_use]
    pub const fn decide(verdict: Verdict) -> Self {
        match verdict.promotable() {
            true => Self::Allow,
            false => Self::Deny,
        }
    }

    /// Deny always overrides allow.
    #[must_use]
    pub const fn deny_overrides(self, other: Self) -> Self {
        match (self, other) {
            (Self::Deny, _) | (_, Self::Deny) => Self::Deny,
            _ => Self::Allow,
        }
    }

    /// An unresolved condition fails closed.
    #[must_use]
    pub const fn on_unknown() -> Self {
        Self::Deny
    }
}
