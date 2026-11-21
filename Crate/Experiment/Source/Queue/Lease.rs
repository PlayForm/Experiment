//! Exclusive hardware measurement lease and signed result release.
//!
//! Grounded in `REQUIREMENTS_V3.md` EXP-001/EXP-007/EXP-010,
//! `ARCHITECTURE.md:56-57` and `playbooks/fanout-5.yaml:69-83`. The lease is
//! granted only by the trusted scheduler; a candidate cannot acquire or extend
//! one. This is a *hardware measurement* lease, distinct from the coordinator
//! task lease.

use crate::coordinator::authority::Authority;
use crate::permission::Permission;

/// Lifecycle phase of a benchmark lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    /// No measurement slot is held.
    Idle,
    /// A runner holds the exclusive slot.
    Held,
    /// The slot was released with a signed manifest.
    Released,
}

/// An exclusive benchmark host lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// Benchmark host identifier.
    pub host: String,
    /// The runner currently holding the lease.
    pub holder: Authority,
    /// Logical sequence at which the lease expires.
    pub deadline_seq: u64,
    /// Current lifecycle phase.
    pub phase: Phase,
    /// Digest of the signed result manifest, set on release.
    pub manifest_sha256: String,
}

impl Lease {
    /// Acquire the lease; only the trusted runner may hold it
    /// (`REQUIREMENTS_V3.md` EXP-001).
    pub fn acquire(host: String, holder: Authority, deadline_seq: u64) -> Result<Self, String> {
        match holder.may_hold_benchmark_lease() {
            Permission::Allow => Ok(Self {
                host,
                holder,
                deadline_seq,
                phase: Phase::Held,
                manifest_sha256: String::new(),
            }),
            Permission::Deny => Err("unauthorized benchmark lease holder".to_string()),
        }
    }

    /// Whether the lease is currently held.
    #[must_use]
    pub const fn is_held(&self) -> bool {
        matches!(self.phase, Phase::Held)
    }

    /// Release the lease with a signed manifest digest.
    pub fn release(mut self, manifest_sha256: String) -> Result<Self, String> {
        match (self.is_held(), manifest_sha256.is_empty()) {
            (true, false) => {
                self.phase = Phase::Released;
                self.manifest_sha256 = manifest_sha256;
                Ok(self)
            }
            _ => Err("lease release requires an active lease and a manifest".to_string()),
        }
    }
}
