//! Patch summary: mechanism hypothesis, risk analysis and anticipated work.
//!
//! Grounded in `AGENT_CONTRACTS.md:26-56` (the patcher prompt contract: one
//! conceptual change, committed patch, short mechanism hypothesis, correctness
//! risk analysis, anticipated workloads; no speedup claims) and
//! `REQUIREMENTS_V3.md` AGT3-004. A summary is metadata around a patch, never a
//! substitute for one: without a committed patch digest it cannot enter the
//! build or benchmark queue.

use crate::permission::Permission;

/// The structured report returned with a committed mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Summary {
    /// Digest of the committed patch; empty means no committed patch exists.
    pub patch_sha256: String,
    /// Digest of the immutable source snapshot.
    pub source_tree_sha256: String,
    /// One-sentence mechanism hypothesis.
    pub hypothesis: String,
    /// Correctness risk notes.
    pub risks: Vec<String>,
    /// Workload families expected to be affected.
    pub anticipated_workloads: Vec<String>,
}

impl Summary {
    /// Whether a committed patch and source snapshot are both present.
    #[must_use]
    pub fn has_committed_patch(&self) -> Permission {
        match self.patch_sha256.is_empty() || self.source_tree_sha256.is_empty() {
            true => Permission::Deny,
            false => Permission::Allow,
        }
    }

    /// Whether this summary claims a speedup. It must not; only the benchmark
    /// service measures (`AGENT_CONTRACTS.md:33`).
    #[must_use]
    pub const fn claims_speedup() -> bool {
        false
    }
}
