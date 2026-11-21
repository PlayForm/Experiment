//! Non-exportable key operations where the platform supports them.
//!
//! `docs/04:41-44` recommends HSM/TPM-backed, proof-of-possession-bound keys
//! for privileged signing, while noting attestation has its own trust
//! assumptions. `docs/08:7` requires the key broker to expose operations, not
//! bytes. This value represents the operation contract; nothing is invoked.

use crate::verdict::Verdict;

/// One privileged operation that never returns key bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonExportable {
    /// Symbolic name of the operation, not executable code.
    pub operation: String,
    /// Digest of the workload attestation that must be current.
    pub attestation_digest: String,
    /// Digest of the policy version authorizing the operation.
    pub policy_digest: String,
}

impl NonExportable {
    /// Record an operation contract from trusted fields.
    #[must_use]
    pub fn of(operation: &str, attestation: &str, policy: &str) -> Self {
        Self {
            operation: operation.to_owned(),
            attestation_digest: attestation.to_owned(),
            policy_digest: policy.to_owned(),
        }
    }

    /// Whether the operation is structurally ready for a trusted broker.
    #[must_use]
    pub fn invocable(&self) -> Verdict {
        Verdict::filled(&self.operation)
            .and(Verdict::digest(&self.attestation_digest))
            .and(Verdict::digest(&self.policy_digest))
    }

    /// Exporting the key is impossible by construction.
    #[must_use]
    pub const fn export() -> Verdict {
        Verdict::Deny
    }
}
