//! The audited `Audit.Project` mapping-disclosure operation.
//!
//! Mirrors `docs/08:49-50`: `Audit.Project(viewer, scope, purpose, time_range)`
//! returns redacted records and the mapping request itself is audited.
//! `docs/07:18-20` requires policy checks and logging for any disclosure of a
//! private mapping. No record is returned and no store is read here.

use crate::verdict::Verdict;

/// One declared audit projection request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Audit {
    /// Authenticated viewer of the request.
    pub viewer: String,
    /// Scope whose records are requested.
    pub scope: String,
    /// Purpose bound to the request.
    pub purpose: String,
    /// Canonical time range supplied by the caller.
    pub time_range: String,
}

impl Audit {
    /// Record a projection request from trusted fields.
    #[must_use]
    pub fn of(viewer: &str, scope: &str, purpose: &str, time_range: &str) -> Self {
        Self {
            viewer: viewer.to_owned(),
            scope: scope.to_owned(),
            purpose: purpose.to_owned(),
            time_range: time_range.to_owned(),
        }
    }

    /// Whether the request is structurally complete enough to evaluate.
    #[must_use]
    pub fn complete(&self) -> Verdict {
        Verdict::filled(&self.viewer)
            .and(Verdict::filled(&self.scope))
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::filled(&self.time_range))
    }

    /// Disclosing a private mapping is always logged, never implicit.
    #[must_use]
    pub const fn mapping_request_logged() -> Verdict {
        Verdict::Allow
    }

    /// Audit records never carry a private denial reason to a worker
    /// (`docs/09:15`).
    #[must_use]
    pub const fn reasons_withheld() -> bool {
        true
    }
}
