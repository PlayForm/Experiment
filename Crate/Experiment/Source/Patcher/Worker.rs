//! A scoped coding worker producing exactly one committed mutation.
//!
//! Grounded in `ARCHITECTURE.md:41-42,64-96`, `AGENT_CONTRACTS.md:10` (Patcher
//! row: sees task source slice, frozen baseline interface, own-lineage
//! feedback; writes one committed mutation; forbidden from runner data, other
//! agent branches and oracle edits), `REQUIREMENTS_V3.md` AGT3-004..009 and
//! `playbooks/fanout-5.yaml:22-47`. A patcher receives no rival patch, no
//! whole-project history and no holdout outcome.

use crate::patcher::scope::Scope;
use crate::patcher::summary::Summary;
use crate::patcher::worktree::Worktree;
use crate::permission::Permission;
use crate::warning::Warning;

/// One warm or freshly spawned patcher worker.
#[derive(Clone, Debug)]
pub struct Worker {
    /// Immutable task identifier assigned by the coordinator.
    pub task_id: String,
    /// The one disjoint mutation scope granted to this worker.
    pub scope: Scope,
    /// The worker's isolated worktree.
    pub worktree: Worktree,
    /// The logical conversation id for its own lineage only.
    pub conversation: String,
}

impl Worker {
    /// Submit a mutation: a saved diff and immutable source snapshot are
    /// mandatory, so a natural-language summary alone cannot enter the build
    /// or benchmark queue (`AGT3-004`, `ARCHITECTURE.md:73-76`).
    pub fn submit(&self, summary: &Summary) -> Result<Summary, Warning> {
        match summary.has_committed_patch() {
            Permission::Allow => Ok(summary.clone()),
            Permission::Deny => Err(Warning::Unreachable {
                service: "patch-submit-requires-committed-patch",
            }),
        }
    }

    /// Whether this worker may read a peer's worktree. It never may
    /// (`AGT3-005`, `AGT3-009`).
    #[must_use]
    pub const fn may_read_peer() -> Permission {
        Permission::Deny
    }

    /// Whether this worker may trigger a benchmark directly. It never may;
    /// measurement is reached only through the signed coordinator plan
    /// (`CONTAINER_MATRIX.md:27-34`, `RUNBOOK.md:94`).
    #[must_use]
    pub const fn may_benchmark() -> Permission {
        Permission::Deny
    }

    /// The scope path assigned to this worker, for the manifest.
    #[must_use]
    pub fn assigned_path(&self) -> &'static str {
        self.scope.path()
    }
}
