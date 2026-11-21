//! Own-lineage feedback delivered to a patcher.
//!
//! Grounded in `ARCHITECTURE.md:90-93,113-117`, `AGENT_CONTRACTS.md:52-57`,
//! `REQUIREMENTS_V3.md` AGT3-006/AGT3-009 and
//! `interfaces/feedback-capsule.v3.schema.json:36-108`. The runner never
//! delivers results to a patcher directly: coordinator ingestion, integrity
//! check, deterministic evaluation, redaction and scoped feedback are
//! mandatory. The capsule states exploratory vs confirmatory status and
//! carries no sealed holdout, peer patch or peer conversation.
//!
//! Exception declared: `Status` is the typed form of this one field and has no
//! independent lifecycle outside `Feedback`.

use crate::permission::Permission;

/// Whether a capsule result is exploratory or confirmatory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// An exploratory development result.
    Exploratory,
    /// A result from a fixed confirmation checkpoint.
    Confirmatory,
}

/// The narrow result view a patcher may receive about its own lineage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Feedback {
    /// The work order this feedback belongs to.
    pub work_order: String,
    /// Evaluation status token, e.g. `benchmark_loss`.
    pub status: String,
    /// Whether the result is exploratory or confirmatory.
    pub status: Status,
    /// Evidence artifact hash this capsule references.
    pub evidence_sha256: String,
    /// Aggregate development hints, sanitized.
    pub hints: Vec<String>,
    /// Minimal failure categories, with no hidden fixture details.
    pub failures: Vec<String>,
}

impl Feedback {
    /// Whether sealed holdout data is present. It never is
    /// (`feedback-capsule.v3.schema.json:103-105`).
    #[must_use]
    pub const fn includes_holdout() -> bool {
        false
    }

    /// Whether this capsule is safe to send to the assigned patcher.
    #[must_use]
    pub fn is_scoped(&self) -> Permission {
        match self.work_order.is_empty() || self.evidence_sha256.is_empty() {
            true => Permission::Deny,
            false => Permission::Allow,
        }
    }
}
