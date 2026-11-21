//! Jurisdiction sensitivity and its fail-closed review requirement.
//!
//! Missing, ambiguous or jurisdiction-sensitive terms become
//! `needs-human-review` or `prohibited` for that use
//! (`docs/20:8-11,18-24`; `examples/v61-license-grant`,
//! `jurisdiction_review_required`).

use crate::permission::Permission;
use crate::switch::Switch;

/// The legal jurisdiction recorded for a license or trade edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Jurisdiction {
    /// Short code naming the governing jurisdiction.
    pub code: String,
    /// Whether an authorized legal owner must review the use.
    pub review_required: Switch,
    /// Digest of the policy that classified the jurisdiction.
    pub policy_digest: String,
}

impl Jurisdiction {
    /// Whether a use in this jurisdiction may proceed.
    #[must_use]
    pub fn permits(&self, reviewed: Switch) -> Permission {
        let cleared = match (self.review_required, reviewed) {
            (Switch::Off, _) => Permission::Allow,
            (Switch::On, Switch::On) => Permission::Allow,
            (Switch::On, Switch::Off) => Permission::Deny,
        };
        cleared
            .and(Permission::filled(&self.code))
            .and(Permission::digest(&self.policy_digest))
    }

    /// An unknown jurisdiction denies by default.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            code: String::new(),
            review_required: Switch::On,
            policy_digest: String::new(),
        }
    }
}
