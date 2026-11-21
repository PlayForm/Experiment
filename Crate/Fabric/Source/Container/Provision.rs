//! Trusted supervisor holder provisioning statement.
//!
//! Mirrors every field of `schemas/holder-provision.schema.json`: holder,
//! image_digest, role, network_class, allowed_secret_classes, attestation_digest,
//! policy_digest and valid_until. `tests/validate.py:34-55` checks the
//! least-privilege `llm_worker` example; the statement is never worker-asserted
//! (`docs/08:29`). Nothing is provisioned here.

use crate::network::Network;
use crate::verdict::Verdict;

/// One provisioned holder statement from a trusted supervisor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provision {
    /// Opaque holder identifier.
    pub holder: String,
    /// Digest of the provisioned image.
    pub image_digest: String,
    /// Canonical service role from the schema enum.
    pub role: String,
    /// Declared network class.
    pub network: Network,
    /// Secret classes the role may hold.
    pub allowed_secret_classes: Vec<String>,
    /// Digest of the workload attestation.
    pub attestation_digest: String,
    /// Digest of the governing policy.
    pub policy_digest: String,
    /// Canonical validity instant.
    pub valid_until: String,
}

impl Provision {
    /// Record a holder statement from trusted fields.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn of(
        holder: &str,
        image: &str,
        role: &str,
        network: Network,
        secrets: Vec<String>,
        attestation: &str,
        policy: &str,
        valid_until: &str,
    ) -> Self {
        Self {
            holder: holder.to_owned(),
            image_digest: image.to_owned(),
            role: role.to_owned(),
            network,
            allowed_secret_classes: secrets,
            attestation_digest: attestation.to_owned(),
            policy_digest: policy.to_owned(),
            valid_until: valid_until.to_owned(),
        }
    }

    /// Whether the statement is structurally complete.
    #[must_use]
    pub fn complete(&self) -> Verdict {
        Verdict::filled(&self.holder)
            .and(Verdict::digest(&self.image_digest))
            .and(Verdict::filled(&self.role))
            .and(Verdict::digest(&self.attestation_digest))
            .and(Verdict::digest(&self.policy_digest))
            .and(Verdict::filled(&self.valid_until))
    }

    /// The worker role may hold no secret class and no network
    /// (`tests/validate.py:36-40`, `README.md:20-21`).
    #[must_use]
    pub fn worker_least_privilege(&self) -> Verdict {
        let no_secrets = self
            .allowed_secret_classes
            .iter()
            .all(|class| class == "none");
        Verdict::of(self.role == "llm_worker")
            .and(self.network.agrees("offline"))
            .and(Verdict::of(no_secrets))
    }
}
