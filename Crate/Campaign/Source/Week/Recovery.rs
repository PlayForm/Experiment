//! Recovery rules from the autonomous-week playbook.
//!
//! Grounded in `playbooks/autonomous-week.yaml:84-91` and
//! `REQUIREMENTS.md:951-955` (Phase 4 gate: crash, DB lock, disk, host restart
//! recovery with no orphaned mutation or lost raw samples). Each rule is a
//! named transition; nothing is scheduled or retried here.

/// A named recovery action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Recovery {
    InterruptedBuild,
    InterruptedMeasurement,
    HeartbeatExpired,
    HostReboot,
    OutOfDisk,
    OracleChanged,
    BaselineDrift,
}

impl Recovery {
    /// All seven rules in playbook order.
    pub const ALL: [Self; 7] = [
        Self::InterruptedBuild,
        Self::InterruptedMeasurement,
        Self::HeartbeatExpired,
        Self::HostReboot,
        Self::OutOfDisk,
        Self::OracleChanged,
        Self::BaselineDrift,
    ];

    /// The exact YAML key.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::InterruptedBuild => "interrupted_build",
            Self::InterruptedMeasurement => "interrupted_measurement",
            Self::HeartbeatExpired => "worker_heartbeat_expired",
            Self::HostReboot => "host_reboot",
            Self::OutOfDisk => "out_of_disk",
            Self::OracleChanged => "oracle_changed",
            Self::BaselineDrift => "baseline_drift",
        }
    }

    /// The playbook's response, as data.
    #[must_use]
    pub const fn action(self) -> &'static str {
        match self {
            Self::InterruptedBuild => "retry_idempotently_if_inputs_match",
            Self::InterruptedMeasurement => "mark_partial_quarantine_then_reschedule",
            Self::HeartbeatExpired => "revoke_lease_and_audit",
            Self::HostReboot => "resume_from_last_durable_state",
            Self::OutOfDisk => "stop_new_builds_checkpoint_and_request_cleanup",
            Self::OracleChanged => "invalidate_comparisons_until_reapproved",
            Self::BaselineDrift => "quarantine_block_and_rerun_reference",
        }
    }
}
