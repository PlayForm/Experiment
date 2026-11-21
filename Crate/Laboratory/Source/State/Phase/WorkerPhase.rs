//! Worker lease states (`docs/06-state-machines.md:42-53`).
//!
//! Idle -> assigned -> running -> staging -> idle, with failed/quarantined/
//! terminated branches. A lease timeout increments the fencing token before
//! another worker may produce authoritative output (`docs/06:92-98`).

/// Worker lease state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkerPhase {
    /// No lease held.
    Idle,
    /// Scheduler granted a lease.
    Assigned,
    /// Supervisor running the offline task.
    Running,
    /// Supervisor staging sealed output.
    Staging,
    /// Task failed.
    Failed,
    /// Host verifier quarantined the worker.
    Quarantined,
    /// Operator terminated it.
    Terminated,
}
