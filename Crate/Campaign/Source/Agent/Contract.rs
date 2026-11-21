//! The agent mission card: fields that must accompany every assignment.
//!
//! Grounded in `playbooks/AGENT_CONTRACT.md:6-23` (mission card), `:41-43`
//! (structured final artifact) and `REQUIREMENTS.md:580-603` (REQ-AGT-01,
//! REQ-AGT-02, REQ-AGT-17). This is a typed record of what an assignment must
//! state; it neither issues assignments nor executes them.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// A bounded, reviewable agent assignment record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    #[serde(rename = "campaign_id")]
    pub campaign_id: String,
    #[serde(rename = "agent_id")]
    pub agent_id: String,
    #[serde(rename = "task_id")]
    pub task_id: String,
    pub attempt: u32,
    #[serde(rename = "parent_mutation_id")]
    pub parent_mutation_id: Option<String>,
    #[serde(rename = "parent_commit")]
    pub parent_commit: String,
    #[serde(rename = "allowed_paths")]
    pub allowed_paths: Vec<String>,
    #[serde(rename = "allowed_tools")]
    pub allowed_tools: Vec<String>,
    #[serde(rename = "worktree_path")]
    pub worktree_path: String,
    pub budget: BTreeMap<String, String>,
    pub deadline: String,
    #[serde(rename = "cpu_allocation")]
    pub cpu_allocation: String,
    #[serde(rename = "hypothesis_id")]
    pub hypothesis_id: String,
    #[serde(rename = "required_correctness_contract")]
    pub required_correctness_contract: String,
    #[serde(rename = "reference_oracle_version")]
    pub reference_oracle_version: String,
    #[serde(rename = "untouched_fixture_ids")]
    pub untouched_fixture_ids: Vec<String>,
    #[serde(rename = "feature_combo_id")]
    pub feature_combo_id: String,
    #[serde(rename = "build_recipe")]
    pub build_recipe: String,
    #[serde(rename = "expected_workloads")]
    pub expected_workloads: Vec<String>,
    #[serde(rename = "acceptance_thresholds")]
    pub acceptance_thresholds: BTreeMap<String, String>,
    #[serde(rename = "preexisting_evidence")]
    pub preexisting_evidence: Vec<String>,
    #[serde(rename = "security_policy")]
    pub security_policy: String,
}

impl Contract {
    /// The 22 keys a complete mission card must carry
    /// (`AGENT_CONTRACT.md:8-23`).
    pub const KEYS: [&'static str; 22] = [
        "campaign_id",
        "agent_id",
        "task_id",
        "attempt",
        "parent_mutation_id",
        "parent_commit",
        "allowed_paths",
        "allowed_tools",
        "worktree_path",
        "budget",
        "deadline",
        "cpu_allocation",
        "hypothesis_id",
        "falsifiable_hypothesis",
        "targeted_stage",
        "required_correctness_contract",
        "reference_oracle_version",
        "untouched_fixture_ids",
        "feature_combo_id",
        "build_recipe",
        "expected_workloads",
        "acceptance_thresholds",
    ];

    /// Whether the assignment forbids uncontrolled shell execution against
    /// personal data (`AGENT_CONTRACT.md:22-23`, REQ-SEC-03).
    #[must_use]
    pub fn shell_is_bounded(&self) -> Permission {
        let named = self
            .allowed_tools
            .iter()
            .any(|tool| tool == "shell" || tool == "unrestricted_shell");
        let policy = self.security_policy.contains("no_uncontrolled_shell");
        match (!named && policy) || (!named && self.allowed_tools.is_empty()) {
            true => Permission::Allow,
            false => Permission::Deny,
        }
    }

    /// Whether the assignment names at least one untouched fixture as
    /// required by the strict mutator instructions (`AGENT_CONTRACT.md:31-33`).
    #[must_use]
    pub fn fixtures_are_pinned(&self) -> Permission {
        match self.untouched_fixture_ids.is_empty() {
            true => Permission::Deny,
            false => Permission::Allow,
        }
    }

    /// Record the structured final artifact the agent must return
    /// (`AGENT_CONTRACT.md:41-43`), without asserting any of its claims.
    #[must_use]
    pub fn artifact_fields(&self) -> Vec<&'static str> {
        vec![
            "mutation_id",
            "commit_sha",
            "patch_digest",
            "modified_paths",
            "risks",
            "tests",
            "reported_limitations",
            "proposed_next_experiment",
        ]
    }
}
