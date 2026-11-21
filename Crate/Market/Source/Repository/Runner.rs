//! The isolated benchmark runner declaration of an adapter.
//!
//! Grounded in `REPOSITORIES.md:17,25` (executable entry points, workload
//! definitions and sample protocol), `SYSTEM.md:93-95,146-148` (exclusive
//! leases with fencing tokens) and `playbooks/research-market-week.yaml:15`
//! (one runner per host). No process is spawned and no lease is taken.

use crate::{Digest, Permission, Warning};

/// The declared exclusive runner for one adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Runner {
    /// Content-addressed runner image digest.
    pub image_digest: Digest,
    /// Declared benchmark entrypoint (`schema:106-112`).
    pub entrypoint: String,
    /// Whether the runner holds an exclusive lease.
    pub exclusive: Permission,
    /// Exclusive runners per host (`playbooks:15`).
    pub runners_per_host: u8,
}

impl Runner {
    /// Validate the entrypoint and host allocation.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.entrypoint.is_empty(),
            self.runners_per_host == crate::BENCHMARK_RUNNERS_PER_HOST,
        ) {
            (false, true) => Ok(()),
            (true, _) => Err(Warning::MissingArtifact {
                digest: "benchmark entrypoint".to_owned(),
            }),
            (_, false) => Err(Warning::CapExceeded {
                cap: "benchmark_runners_per_host",
                id: self.entrypoint.clone(),
            }),
        }
    }

    /// A runner cannot be shared by noisy CI workloads
    /// (`REPOSITORIES.md:70`, `SYSTEM.md:53`).
    #[must_use]
    pub const fn may_share_with_ci() -> Permission {
        Permission::Deny
    }
}
