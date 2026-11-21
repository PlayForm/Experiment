//! Corrections that supersede facts without mutating them.
//!
//! A later observation may correct an earlier one while history, provenance
//! and the original recording remain intact. Supersession is a typed link, not
//! an in-place edit (`docs/21:35-37`, `docs/17:61`; `docs/25:21`).

use super::fact::Fact;
use crate::permission::Permission;

/// A signed link from a corrected fact to its replacement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Supersede {
    /// Identifier of the fact being corrected.
    pub old_id: String,
    /// Identifier of the correcting fact.
    pub new_id: String,
    /// Ledger sequence authorizing the correction.
    pub sequence: u64,
    /// Digest of the policy that permitted the correction.
    pub policy_digest: String,
}

impl Supersede {
    /// Whether the correction is admissible against the two facts.
    #[must_use]
    pub fn permits(&self, old: &Fact, new: &Fact) -> Permission {
        Permission::filled(&self.old_id)
            .and(Permission::filled(&self.new_id))
            .and(Permission::digest(&self.policy_digest))
            .and(Permission::of(self.sequence > 0))
            .and(Permission::of(old.fact_id == self.old_id))
            .and(Permission::of(new.fact_id == self.new_id))
            .and(Permission::of(
                new.supersedes_fact_id.as_deref() == Some(self.old_id.as_str()),
            ))
    }

    /// The preserved history; the old fact is returned alongside the new.
    #[must_use]
    pub fn preserved<'a>(&self, old: &'a Fact, new: &'a Fact) -> Vec<&'a Fact> {
        match self.permits(old, new) {
            Permission::Allow => vec![old, new],
            Permission::Deny => Vec::new(),
        }
    }
}
