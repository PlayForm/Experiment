//! Resource policy and caps for the bounded autonomous week.
//!
//! Grounded in `playbooks/autonomous-week.yaml:8-22` (every `resource_policy`
//! key), `campaign.v2.json:24-35` (the larger campaign budgets) and
//! REQ-AGT-16 (`REQUIREMENTS.md:596-599`). All limits are typed values;
//! nothing here starts a timer, opens a lease or checks a disk.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// The bounded resource policy of a campaign week.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Budget {
    #[serde(rename = "duration_hours")]
    pub duration_hours: u32,
    #[serde(rename = "require_dedicated_measurement_lease", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub measurement_lease: Permission,
    #[serde(rename = "max_mutations")]
    pub max_mutations: u32,
    #[serde(rename = "max_builds", default)]
    pub max_builds: u32,
    #[serde(rename = "max_trial_blocks", default)]
    pub max_trial_blocks: u32,
    #[serde(rename = "max_concurrent_agents")]
    pub max_concurrent_agents: u32,
    #[serde(rename = "max_concurrent_builds")]
    pub max_concurrent_builds: u32,
    #[serde(rename = "max_concurrent_timing_jobs_per_host")]
    pub max_timing_jobs_per_host: u32,
    #[serde(rename = "max_build_attempts_per_mutation")]
    pub max_build_attempts: u32,
    #[serde(rename = "max_agent_attempts_per_mutation", default)]
    pub max_agent_attempts: u32,
    #[serde(rename = "max_disk_gb", default)]
    pub max_disk_gb: u32,
    #[serde(rename = "stop_at_free_disk_gb")]
    pub min_free_disk_gb: u32,
    #[serde(rename = "max_trial_runtime_seconds", default)]
    pub max_trial_runtime_seconds: u32,
    #[serde(rename = "checkpoint_after_every_transition", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub checkpoint_every_transition: Permission,
    #[serde(rename = "disallow_agent_docker_socket", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub disallow_docker_socket: Permission,
    #[serde(rename = "no_network_in_runner", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub no_runner_network: Permission,
    #[serde(rename = "no_direct_release_or_merge", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub no_direct_release: Permission,
    #[serde(rename = "require_human_approval_for_oracle_changes", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub human_approval_for_oracle: Permission,
    #[serde(rename = "fail_closed_on_host_health_alarm", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub fail_closed_on_alarm: Permission,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            duration_hours: crate::WEEK_HOURS,
            measurement_lease: Permission::Allow,
            max_mutations: 160,
            max_builds: 500,
            max_trial_blocks: 1_200,
            max_concurrent_agents: 3,
            max_concurrent_builds: 2,
            max_timing_jobs_per_host: 1,
            max_build_attempts: 3,
            max_agent_attempts: 3,
            max_disk_gb: 200,
            min_free_disk_gb: 25,
            max_trial_runtime_seconds: 600,
            checkpoint_every_transition: Permission::Allow,
            disallow_docker_socket: Permission::Allow,
            no_runner_network: Permission::Allow,
            no_direct_release: Permission::Allow,
            human_approval_for_oracle: Permission::Allow,
            fail_closed_on_alarm: Permission::Allow,
        }
    }
}

impl Budget {
    /// Recognised YAML keys (`autonomous-week.yaml:9-22`).
    pub const KEYS: [&'static str; 19] = [
        "duration_hours",
        "require_dedicated_measurement_lease",
        "max_mutations",
        "max_builds",
        "max_trial_blocks",
        "max_concurrent_agents",
        "max_concurrent_builds",
        "max_concurrent_timing_jobs_per_host",
        "max_build_attempts_per_mutation",
        "max_agent_attempts_per_mutation",
        "max_disk_gb",
        "stop_at_free_disk_gb",
        "max_trial_runtime_seconds",
        "checkpoint_after_every_transition",
        "disallow_agent_docker_socket",
        "no_network_in_runner",
        "no_direct_release_or_merge",
        "require_human_approval_for_oracle_changes",
        "fail_closed_on_host_health_alarm",
    ];

    /// Warn on a policy that cannot protect measurement quality
    /// (LAW-005, LAW-010, `campaign.v2.json:30`).
    pub fn validate(&self, warnings: &mut Vec<crate::Warning>) {
        match self.max_timing_jobs_per_host {
            1 => {}
            _ => warnings.push(crate::Warning::Bound {
                domain: "resource_policy",
                key: "max_concurrent_timing_jobs_per_host must be 1".to_owned(),
            }),
        }
        match self.measurement_lease {
            Permission::Allow => {}
            Permission::Deny => warnings.push(crate::Warning::Bound {
                domain: "resource_policy",
                key: "require_dedicated_measurement_lease".to_owned(),
            }),
        }
        match self.min_free_disk_gb {
            0 => warnings.push(crate::Warning::Range {
                domain: "resource_policy",
                key: "stop_at_free_disk_gb".to_owned(),
            }),
            _ => {}
        }
    }

    /// Whether the campaign stops cleanly when a bound is reached
    /// (`REQUIREMENTS.md:623-625`).
    #[must_use]
    pub fn stops_cleanly(&self) -> bool {
        self.fail_closed_on_alarm == Permission::Allow
            && self.checkpoint_every_transition == Permission::Allow
    }
}
