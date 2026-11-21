//! Per-task isolated worktree: the only writable source a patcher sees.
//!
//! Grounded in `ARCHITECTURE.md:69-73,119-125`, `REQUIREMENTS_V3.md`
//! AGT3-004/AGT3-005 and `CONTAINER_MATRIX.md:36-47` (writable assigned
//! overlay/worktree only, no sibling mounts, no Docker socket, no credentials).
//! Worktrees are scratch, never a source of truth (`STORAGE.md:25-37`).

/// An isolated source worktree assigned to exactly one worker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Worktree {
    /// The task id that owns this worktree.
    pub task_id: String,
    /// The immutable parent source commit this worktree was forked from.
    pub parent_commit: String,
    /// Digest of the exact source snapshot visible to the worker.
    pub snapshot_sha256: String,
    /// The path prefix this worktree exposes to its assigned worker.
    pub allowed_prefix: String,
    /// Identifier of the view policy restricting the visible files.
    pub view_policy_id: String,
}

impl Worktree {
    /// Whether a path lies inside this worktree's assigned scope. The check is
    /// a pure prefix comparison; no filesystem is touched.
    #[must_use]
    pub fn contains(&self, path: &str) -> bool {
        path.starts_with(&self.allowed_prefix)
    }

    /// Whether this worktree may be written by a different task. It never may.
    #[must_use]
    pub fn writable_by(&self, other_task: &str) -> bool {
        other_task == self.task_id
    }
}
