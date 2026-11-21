//! Pinned policy evaluation with deny-overrides-allow.
//!
//! Grounded in `docs/26-acceptance-and-build-plan.md:9-13` (embedded Cedar
//! policy over `(principal, action, resource, context)`; deny overrides allow,
//! any unknown fails closed; Cedar does not itself decide legal compatibility),
//! `docs/17-v61-constitution.md:58-63` and
//! `docs/25-failure-analysis.md` F15 (a policy-service error must never be read
//! as permission). No policy expression is evaluated here; only a pinned
//! decision already recorded by trusted software is read.

use crate::{Digest, Permission};

/// A pinned policy and the decision it authorizes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    /// Signed policy version.
    pub version: u32,
    /// Digest of the canonical policy bytes.
    pub digest: Digest,
    /// Whether an explicitly recorded allow survives evaluation.
    pub allow_recorded: Permission,
    /// Whether an explicit deny was recorded for this scope.
    pub deny_recorded: Permission,
}

impl Policy {
    /// Pin a policy with no recorded allow or deny: fail closed.
    #[must_use]
    pub const fn of(version: u32, digest: Digest) -> Self {
        Self {
            version,
            digest,
            allow_recorded: Permission::Deny,
            deny_recorded: Permission::Deny,
        }
    }

    /// Evaluate against the pinned decision; deny overrides allow.
    #[must_use]
    pub const fn evaluate(&self) -> Permission {
        match self.deny_recorded {
            Permission::Allow => Permission::Deny,
            Permission::Deny => self.allow_recorded,
        }
    }

    /// An unknown or unavailable policy service never permits.
    #[must_use]
    pub const fn on_unavailable(&self) -> Permission {
        Permission::Deny
    }
}
