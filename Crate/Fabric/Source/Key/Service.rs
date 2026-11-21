//! A scoped online workload-identity key handle.
//!
//! Mirrors the workload-identity issuer of `docs/04:2`, the schema-factory and
//! identity-issuer rows of `docs/08:5,8`, and the requirement that only trusted
//! Rust services receive short-lived mTLS identities (`README.md:19-21`).
//! The value is a handle, not key material.

use crate::verdict::Verdict;
use super::hierarchy::Hierarchy;

/// One online service key handle bound to a workload and epoch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Service {
    /// Authority this handle belongs to.
    pub authority: Hierarchy,
    /// Epoch in which the handle is current.
    pub epoch: u64,
    /// Opaque handle issued by the broker, never raw bytes.
    pub handle: String,
    /// Digest of the workload identity this handle is scoped to.
    pub workload_digest: String,
}

impl Service {
    /// Record a service handle from trusted issuance fields.
    #[must_use]
    pub fn of(authority: Hierarchy, epoch: u64, handle: &str, workload: &str) -> Self {
        Self {
            authority,
            epoch,
            handle: handle.to_owned(),
            workload_digest: workload.to_owned(),
        }
    }

    /// Whether the handle is structurally scoped to one workload and epoch.
    #[must_use]
    pub fn scoped(&self) -> Verdict {
        Verdict::filled(&self.handle)
            .and(Verdict::digest(&self.workload_digest))
            .and(Verdict::of(self.epoch > 0))
    }

    /// A broker handle is not a returned secret and cannot sign for another
    /// authority (`docs/04:3`, `docs/08:7`).
    #[must_use]
    pub fn crosses_authority(&self, other: &Self) -> Verdict {
        self.authority.separate_from(&other.authority)
    }
}
