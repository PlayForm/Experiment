//! A signed attestation of a receiving holder's capability.
//!
//! Mirrors `examples/v61-holder-capability.example.json` and the inventory
//! step of the two-phase protocol. Capacity claimed by a model is never
//! believed; a fresh signed deadline is required
//! (`docs/19:23-27,87-93`).

use crate::network::Network;
use crate::permission::Permission;
use crate::switch::Switch;
use crate::warning::Warning;

/// The attested environment of one artifact holder.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Holder {
    /// `holder_id` used as the opaque local target reference.
    pub holder_id: String,
    /// `holder_role`, e.g. `offline-agent` in the pinned example.
    pub holder_role: String,
    /// `environment_digest` of the approved offline image.
    pub environment_digest: String,
    /// `trust_domain` scoped to one tenant.
    pub trust_domain: String,
    /// `tenant_id` for envelope-key and CAS namespace isolation.
    pub tenant_id: String,
    /// `media_types` the holder can open safely.
    pub media_types: Vec<String>,
    /// `accepted_classes` of artifact confidentiality.
    pub accepted_classes: Vec<String>,
    /// `supported_purposes` under which bytes may be mounted.
    pub supported_purposes: Vec<String>,
    /// `max_bytes` the holder can reserve.
    pub max_bytes: u64,
    /// `free_bytes` observed at attestation time, not claimed by a worker.
    pub free_bytes: u64,
    /// `encryption_at_rest` capability (`true` in the example).
    pub encryption_at_rest: Switch,
    /// `ephemeral_storage` guarantee for scratch bytes.
    pub ephemeral_storage: Switch,
    /// `network_mode`, required to be `none` for an offline worker.
    pub network_mode: Network,
    /// `credential_fingerprint` of the attested supervisor.
    pub credential_fingerprint: String,
    /// `attestation_digest` over the signed inventory report.
    pub attestation_digest: String,
    /// `expires_at` freshness deadline of this report.
    pub expires_at: String,
}

impl Holder {
    /// Whether the attestation can qualify a destination for staging.
    #[must_use]
    pub fn attests(&self) -> Permission {
        Permission::filled(&self.holder_id)
            .and(Permission::digest(&self.environment_digest))
            .and(Permission::digest(&self.attestation_digest))
            .and(Permission::filled(&self.expires_at))
            .and(self.network_mode.offline())
            .and(self.encryption_at_rest.permit())
            .and(Permission::of(
                self.free_bytes > 0 && self.free_bytes <= self.max_bytes,
            ))
    }

    /// Whether this holder supports a named purpose and class together.
    #[must_use]
    pub fn supports(&self, purpose: &str, class: &str) -> Permission {
        Permission::of(self.supported_purposes.iter().any(|item| item == purpose))
            .and(Permission::of(
                self.accepted_classes.iter().any(|item| item == class),
            ))
    }

    /// Capacity and format findings for an authorized operator.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let missing = [
            ("holder id", self.holder_id.is_empty()),
            ("trust domain", self.trust_domain.is_empty()),
            ("tenant", self.tenant_id.is_empty()),
            ("expiry", self.expires_at.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, absent)| {
            absent.then(|| Warning::Missing { domain: field })
        });
        let digests = [
            ("environment_digest", self.environment_digest.as_str()),
            ("attestation_digest", self.attestation_digest.as_str()),
        ]
        .into_iter()
        .filter_map(|(field, value)| {
            (!crate::digest::Digest::of(value).is_valid()).then(|| Warning::DigestShape {
                field,
                detail: crate::digest::Digest::of(value).describe(),
            })
        });
        let offline = match self.network_mode.offline() {
            Permission::Allow => None,
            Permission::Deny => Some(Warning::Malformed {
                domain: "network_mode",
                detail: "an offline holder must have no routes".to_owned(),
            }),
        };
        let capacity = match Permission::of(self.free_bytes <= self.max_bytes) {
            Permission::Allow => None,
            Permission::Deny => Some(Warning::Malformed {
                domain: "free_bytes",
                detail: "attested capacity exceeds the holder maximum".to_owned(),
            }),
        };
        missing
            .chain(digests)
            .chain(offline)
            .chain(capacity)
            .collect()
    }
}
