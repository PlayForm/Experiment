//! Recipient cohorts and the only aggregate disclosure an embargo allows.
//!
//! A rule may bind recipient groups; a broker may publish anonymous progress
//! counts only when a signed disclosure rule explicitly permits it. Peer
//! identity never appears in that aggregate
//! (`docs/18:45-53`, `docs/20:47-53`).

use crate::permission::Permission;
use crate::switch::Switch;

/// A named group of recipients under one embargo or assignment policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cohort {
    /// `cohort_id` of the group.
    pub cohort_id: String,
    /// `embargo_id` linking the group to a rule.
    pub embargo_id: String,
    /// Opaque member references retained only in trusted state.
    pub members: Vec<String>,
    /// Whether an anonymous count may leave the broker.
    pub aggregate_allowed: Switch,
}

impl Cohort {
    /// Whether a member belongs to this cohort.
    #[must_use]
    pub fn contains(&self, member: &str) -> bool {
        self.members.iter().any(|item| item == member)
    }

    /// Whether an anonymous count may be published at all.
    #[must_use]
    pub fn disclose_count(&self) -> Permission {
        self.aggregate_allowed
            .permit()
            .and(Permission::filled(&self.cohort_id))
    }

    /// An anonymous count with no member identifiers attached.
    #[must_use]
    pub fn anonymous_count(&self) -> usize {
        match self.disclose_count() {
            Permission::Allow => self.members.len(),
            Permission::Deny => 0,
        }
    }
}
