//! A CA-attested identity certificate binding.
//!
//! Grounded in `security/src/crypto.rs:44-48` (`AttestedIdentity`: subject plus
//! authority signature), `crypto.rs:27` (registration domain separation),
//! `crypto.rs:103-108` (attestation signs the domain-prefixed subject JSON),
//! `crypto.rs:109-129` (verification of that binding) and
//! `storage.rs:183-194` (the TLS leaf certificate carrying both demo EKUs).
//! The certificate here records which facts are bound; it verifies no signature.

use serde::{Deserialize, Serialize};

use crate::identity::identity::Identity;
use crate::permission::Permission;

/// The signed binding between a subject identity and the issuing authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    /// The attested subject.
    pub subject: Identity,
    /// Base64 Ed25519 signature over the domain-prefixed subject JSON.
    pub authority_signature_b64: String,
    /// Subject alternative name of the demo TLS leaf (`storage.rs:183`).
    pub san: String,
    /// Common name prefix of the demo TLS leaf (`storage.rs:186`).
    pub common_name_prefix: String,
    /// The two demo EKUs (`storage.rs:187-190`).
    pub extended_key_usages: Vec<String>,
}

impl Certificate {
    /// Domain separator for identity attestation (`crypto.rs:27`).
    pub const DOMAIN: &'static [u8] = b"prun-lab/v4/identity-attestation/ed25519\0";

    /// Construct a certificate descriptor from an attested subject and the
    /// prototype's demo TLS leaf parameters.
    #[must_use]
    pub fn prototype(subject: Identity, signature_b64: String) -> Self {
        let san = format!("{}.lab.internal", subject.identity);
        let common_name_prefix = "run-agent:".to_owned();
        let extended_key_usages = ["clientAuth", "serverAuth"]
            .iter()
            .map(|name| (*name).to_owned())
            .collect();
        Self {
            subject,
            authority_signature_b64: signature_b64,
            san,
            common_name_prefix,
            extended_key_usages,
        }
    }

    /// The ordered fields covered by the authority signature
    /// (`crypto.rs:105-107` signs the whole serialized subject).
    #[must_use]
    pub const fn signed_field_count() -> usize {
        9
    }

    /// Whether the demo leaf grants a named extended key usage
    /// (`storage.rs:187-190`).
    #[must_use]
    pub fn grants_eku(&self, name: &str) -> bool {
        self.extended_key_usages.iter().any(|eku| eku == name)
    }

    /// Certificate issuance attests identity binding only. It never attests
    /// benchmark correctness (`V4_TRUST_BRIDGE.md:57-63`).
    #[must_use]
    pub const fn attests_scientific_validity() -> Permission {
        Permission::Deny
    }
}
