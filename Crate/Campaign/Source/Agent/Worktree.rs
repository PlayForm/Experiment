//! The isolated worktree a mutator is allowed to edit.
//!
//! Grounded in `REQUIREMENTS.md:558-566`, `:94` (LAW-014), `:580-603`
//! (REQ-AGT-02, REQ-AGT-04, REQ-AGT-05) and `campaign.v2.json:225`
//! (`worktree_root`). A worktree here is a path and a claim record; no
//! directory is created and no branch is checked out.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// A single-agent, single-mutation worktree claim.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Worktree {
    /// Declared root, for example `.prun-lab/worktrees` (`campaign.v2.json:225`).
    #[serde(rename = "root")]
    pub root: String,
    /// Per-mutation leaf path.
    pub path: String,
    /// The commit the worktree was created from.
    #[serde(rename = "base_commit")]
    pub base_commit: String,
    /// The agent that currently owns this worktree.
    #[serde(rename = "owner_agent_id")]
    pub owner_agent_id: String,
    /// Whether the worktree is reserved for one mutation at a time.
    pub exclusive: Permission,
    /// Whether the worktree is isolated from the main branch.
    pub isolated: Permission,
}

impl Worktree {
    /// Whether this claim may be granted to a second mutation.
    #[must_use]
    pub fn accepts(&self, agent_id: &str) -> Permission {
        match self.exclusive.and(self.isolated) {
            Permission::Allow => match self.owner_agent_id == agent_id || self.owner_agent_id.is_empty() {
                true => Permission::Allow,
                false => Permission::Deny,
            },
            Permission::Deny => Permission::Deny,
        }
    }

    /// The path is data; the model never touches the filesystem.
    #[must_use]
    pub fn leaf(&self) -> String {
        format!("{}/{}", self.root, self.path)
    }
}
