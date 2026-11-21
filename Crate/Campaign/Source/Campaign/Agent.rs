//! Agent configuration in the v2 campaign document.
//!
//! Grounded in `campaign.v2.json:213-231`, the schema at
//! `schemas/campaign.v2.schema.json:635-679`, `REQUIREMENTS.md:556-603`
//! (REQ-AGT-01..18) and `AGENT_CONTRACT.md:1-4`. The playbook path names a
//! design contract; no YAML is read, emitted or executed here.
//!
//! Exception declared: `Provider` is declared beside `Agent` because it is
//! the typed form of this one field and has no independent lifecycle.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// The replaceable local adapter kind (`campaign.v2.json:214`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Provider {
    /// A local command adapter; every external model is optional.
    LocalCommand,
    /// Any other provider is a design-only token, not implemented.
    Named,
}

impl Provider {
    /// The JSON token (`campaign.v2.json:214`).
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::LocalCommand => "local_command_adapter",
            Self::Named => "named",
        }
    }
}

/// The agent section of the campaign document.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    pub provider: String,
    pub roles: Vec<String>,
    #[serde(rename = "worktree_root")]
    pub worktree_root: String,
    pub playbook: String,
    #[serde(rename = "require_hypothesis_card", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub require_hypothesis: Permission,
    #[serde(rename = "require_single_concept_patch", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub require_single_concept: Permission,
    #[serde(rename = "require_committed_snapshot", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub require_snapshot: Permission,
    #[serde(default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub auto_merge: Permission,
}

impl Agent {
    /// Recognised keys (`schemas/campaign.v2.schema.json:671-677`).
    pub const KEYS: [&'static str; 8] = [
        "provider",
        "roles",
        "worktree_root",
        "playbook",
        "require_hypothesis_card",
        "require_single_concept_patch",
        "require_committed_snapshot",
        "auto_merge",
    ];

    /// The ten roles named in the design (`REQUIREMENTS.md:559-578`), which
    /// include the eight used by the example (`campaign.v2.json:215-224`).
    pub const ROLES: [&'static str; 10] = [
        "scout",
        "decomposer",
        "mutator",
        "oracle",
        "builder",
        "runner",
        "analyst",
        "planner-tuner",
        "integrator",
        "curator",
    ];

    /// Whether agents are forbidden from merging automatically
    /// (REQ-SEC-07, REQ-AGT-15).
    #[must_use]
    pub fn merge_is_manual(&self) -> Permission {
        self.auto_merge
    }

    /// Whether the agent section satisfies the strict mutation rules
    /// (`AGENT_CONTRACT.md:27-40`).
    #[must_use]
    pub fn is_bounded(&self) -> bool {
        self.require_single_concept == Permission::Allow
            && self.require_snapshot == Permission::Allow
            && self.auto_merge == Permission::Deny
    }
}
