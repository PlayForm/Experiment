//! One Cedar-like principal/action/resource/purpose rule.
//!
//! `docs/05:34-37` describes a Cedar-style engine evaluating principal,
//! action, resource and context. `docs/04:33-38` adds purpose, recipient,
//! policy version and freshness. A rule is a typed value; it authorizes
//! nothing on its own.

use crate::verdict::Verdict;

/// One structural entitlement rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    /// Private semantic ID of the rule.
    pub rule_id: String,
    /// Authenticated principal the rule applies to.
    pub principal: String,
    /// Named action, e.g. `resolve` or `decrypt`.
    pub action: String,
    /// Opaque resource handle.
    pub resource: String,
    /// Purpose binding from the grant.
    pub purpose: String,
    /// Digest of the policy version containing this rule.
    pub policy_digest: String,
}

impl Rule {
    /// Record a signed rule from trusted fields.
    #[must_use]
    pub fn of(
        rule_id: &str,
        principal: &str,
        action: &str,
        resource: &str,
        purpose: &str,
        policy: &str,
    ) -> Self {
        Self {
            rule_id: rule_id.to_owned(),
            principal: principal.to_owned(),
            action: action.to_owned(),
            resource: resource.to_owned(),
            purpose: purpose.to_owned(),
            policy_digest: policy.to_owned(),
        }
    }

    /// Whether the supplied request structurally matches this rule.
    #[must_use]
    pub fn matches(&self, principal: &str, action: &str, resource: &str, purpose: &str) -> Verdict {
        Verdict::of(self.principal == principal)
            .and(Verdict::of(self.action == action))
            .and(Verdict::of(self.resource == resource))
            .and(Verdict::of(self.purpose == purpose))
            .and(Verdict::digest(&self.policy_digest))
    }

    /// Whether the rule is complete enough to be installed in the registry.
    #[must_use]
    pub fn complete(&self) -> Verdict {
        Verdict::filled(&self.rule_id)
            .and(Verdict::filled(&self.principal))
            .and(Verdict::filled(&self.action))
            .and(Verdict::filled(&self.resource))
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::digest(&self.policy_digest))
    }
}
