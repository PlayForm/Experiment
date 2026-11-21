//! The campaign safety block: seven explicit permissions.
//!
//! Grounded in `campaign.v2.json:36-44`, the schema's required safety keys
//! (`schemas/campaign.v2.schema.json:187-218`), LAW-014 (`REQUIREMENTS.md:92-94`)
//! and REQ-SEC-02 / REQ-SEC-07 / REQ-SEC-08 (`REQUIREMENTS.md:915-927`).
//! Every boolean is a typed `Permission` whose default is `Deny`, so an
//! omitted safety field fails closed rather than granting capability.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// Safety policy for an unattended campaign.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Safety {
    #[serde(rename = "require_human_release_approval", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub require_human_release_approval: Permission,
    #[serde(rename = "allow_agent_oracle_edits", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub allow_agent_oracle_edits: Permission,
    #[serde(rename = "allow_network_in_bench_runner", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub allow_network_in_bench_runner: Permission,
    #[serde(rename = "allow_privileged_benchmark_runner", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub allow_privileged_benchmark_runner: Permission,
    #[serde(rename = "allow_agent_docker_socket", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub allow_agent_docker_socket: Permission,
    #[serde(rename = "allow_unapproved_external_commands", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub allow_unapproved_external_commands: Permission,
    #[serde(rename = "stop_on_host_health_alarm", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub stop_on_host_health_alarm: Permission,
}

impl Default for Safety {
    fn default() -> Self {
        Self {
            require_human_release_approval: Permission::Allow,
            allow_agent_oracle_edits: Permission::Deny,
            allow_network_in_bench_runner: Permission::Deny,
            allow_privileged_benchmark_runner: Permission::Deny,
            allow_agent_docker_socket: Permission::Deny,
            allow_unapproved_external_commands: Permission::Deny,
            stop_on_host_health_alarm: Permission::Allow,
        }
    }
}

impl Safety {
    /// Recognised keys (`schemas/campaign.v2.schema.json:212-216`).
    pub const KEYS: [&'static str; 7] = [
        "require_human_release_approval",
        "allow_agent_oracle_edits",
        "allow_network_in_bench_runner",
        "allow_privileged_benchmark_runner",
        "allow_agent_docker_socket",
        "allow_unapproved_external_commands",
        "stop_on_host_health_alarm",
    ];

    /// List capabilities that were left open, so the caller can warn and
    /// degrade instead of assuming a safe default.
    #[must_use]
    pub fn open_capabilities(&self) -> Vec<&'static str> {
        [
            (Self::KEYS[1], self.allow_agent_oracle_edits),
            (Self::KEYS[2], self.allow_network_in_bench_runner),
            (Self::KEYS[3], self.allow_privileged_benchmark_runner),
            (Self::KEYS[4], self.allow_agent_docker_socket),
            (Self::KEYS[5], self.allow_unapproved_external_commands),
        ]
        .into_iter()
        .filter(|(_, permission)| *permission == Permission::Allow)
        .map(|(name, _)| name)
        .collect()
    }

    /// Whether the campaign still requires a human release approval
    /// (REQ-SEC-07).
    #[must_use]
    pub fn release_is_supervised(&self) -> Permission {
        self.require_human_release_approval
    }

    /// Whether a host health alarm must pause rather than continue
    /// (`AGENT_CONTRACT.md:88-89`, `autonomous-week.yaml:22`).
    #[must_use]
    pub fn fails_closed_on_alarm(&self) -> Permission {
        self.stop_on_host_health_alarm
    }
}
