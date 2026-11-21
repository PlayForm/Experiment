//! The trusted policy registry holding semantic entitlement IDs only.
//!
//! `docs/08:9` keeps the policy engine free of data keys and schema mappings.
//! `docs/08:28-30` requires caller identity, purpose and proof for resolution,
//! and `docs/09:68-78` lists the cross-field checks a schema cannot prove. A
//! registry holds rule values; it evaluates structure, not cryptography.

use crate::verdict::Verdict;
use super::rule::Rule;

/// A signed, versioned set of rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registry {
    /// Digest binding the whole policy version.
    pub policy_digest: String,
    /// Epoch in which the policy version is current.
    pub epoch: u64,
}

impl Registry {
    /// Record a policy version identity.
    #[must_use]
    pub fn of(policy_digest: &str, epoch: u64) -> Self {
        Self {
            policy_digest: policy_digest.to_owned(),
            epoch,
        }
    }

    /// Evaluate whether any supplied rule matches the request and belongs to
    /// this policy version. Iteration is a named chain.
    #[must_use]
    pub fn authorizes<'a>(
        &self,
        rules: impl IntoIterator<Item = &'a Rule>,
        principal: &str,
        action: &str,
        resource: &str,
        purpose: &str,
    ) -> Verdict {
        let matching = rules.into_iter().any(|rule| {
            rule.complete() == Verdict::Allow
                && rule.policy_digest == self.policy_digest
                && rule.matches(principal, action, resource, purpose) == Verdict::Allow
        });
        self.current().and(Verdict::of(matching))
    }

    /// Whether the registry view is structurally current. No clock is read;
    /// the caller supplies the epoch comparison as a value.
    #[must_use]
    pub fn current(&self) -> Verdict {
        Verdict::digest(&self.policy_digest).and(Verdict::of(self.epoch > 0))
    }

    /// A denied decision never carries a reason to a worker
    /// (`docs/07:28-30`).
    #[must_use]
    pub const fn reasons_are_private() -> bool {
        true
    }
}
