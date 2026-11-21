//! Security and authority bounds that a coding agent never crosses.
//!
//! Grounded in `AGENT_CONTRACT.md:25-45` (strict mutator instructions,
//! "do not self-promote"), `REQUIREMENTS.md:894-927` (REQ-SEC-01..10),
//! `:580-603` (REQ-AGT-17, REQ-AGT-18) and LAW-014. Each bound is a typed
//! permission; the default `Deny` fails closed.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// The authority boundary of an untrusted coding agent.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bound {
    #[serde(rename = "docker_socket")]
    pub docker_socket: Permission,
    pub network: Permission,
    #[serde(rename = "oracle_edits")]
    pub oracle_edits: Permission,
    #[serde(rename = "fixture_edits")]
    pub fixture_edits: Permission,
    #[serde(rename = "main_branch")]
    pub main_branch: Permission,
    #[serde(rename = "self_promotion")]
    pub self_promotion: Permission,
    #[serde(rename = "release_rights")]
    pub release_rights: Permission,
    #[serde(rename = "privileged_runner")]
    pub privileged_runner: Permission,
}

impl Default for Bound {
    fn default() -> Self {
        Self {
            docker_socket: Permission::Deny,
            network: Permission::Deny,
            oracle_edits: Permission::Deny,
            fixture_edits: Permission::Deny,
            main_branch: Permission::Deny,
            self_promotion: Permission::Deny,
            release_rights: Permission::Deny,
            privileged_runner: Permission::Deny,
        }
    }
}

impl Bound {
    /// The eight bounds that must be `Deny` for a restricted agent
    /// (`REQ-SEC-02`, `AGENT_CONTRACT.md:44-45`).
    pub const NAMES: [&'static str; 8] = [
        "docker_socket",
        "network",
        "oracle_edits",
        "fixture_edits",
        "main_branch",
        "self_promotion",
        "release_rights",
        "privileged_runner",
    ];

    /// List the bounds left open, so a caller can warn and degrade rather
    /// than assuming the agent is safe.
    #[must_use]
    pub fn open(&self) -> Vec<&'static str> {
        [
            (Self::NAMES[0], self.docker_socket),
            (Self::NAMES[1], self.network),
            (Self::NAMES[2], self.oracle_edits),
            (Self::NAMES[3], self.fixture_edits),
            (Self::NAMES[4], self.main_branch),
            (Self::NAMES[5], self.self_promotion),
            (Self::NAMES[6], self.release_rights),
            (Self::NAMES[7], self.privileged_runner),
        ]
        .into_iter()
        .filter(|(_, permission)| *permission == Permission::Allow)
        .map(|(name, _)| name)
        .collect()
    }

    /// Only the trusted coordinator may promote a candidate
    /// (REQ-AGT-18, `AGENT_CONTRACT.md:44-45`).
    #[must_use]
    pub fn coordinator_only_promotion(&self) -> Permission {
        self.self_promotion
    }
}
