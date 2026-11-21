//! The identity issuance pipeline as a named sequence of steps.
//!
//! Grounded in `security/src/storage.rs:163-220`: `issue` generates an Ed25519
//! signing key and an HPKE encryption key, signs a short-lived TLS leaf with
//! the offline root, attests a `PublicIdentity`, writes the registry record and
//! journals an `IDENTITY_ISSUED` audit record. The demo leaf carries both client
//! and server EKUs; production must issue purpose-separated leaves
//! (`storage.rs:181-190`). No key is generated and no certificate is signed here.

/// One step in the prototype's issuance pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Issuer {
    /// Generate the Ed25519 message-signing key (`storage.rs:176`).
    GenerateSigningKey,
    /// Generate the HPKE decryption key (`storage.rs:177`).
    GenerateEncryptionKey,
    /// Sign a short-lived TLS leaf with the offline root (`storage.rs:178-194`).
    IssueLeafCertificate,
    /// Sign the identity binding with the message authority (`crypto.rs:103-108`).
    AttestIdentity,
    /// Append the signed issuance journal record (`storage.rs:212-218`).
    JournalIssuance,
}

impl Issuer {
    /// The complete pipeline in execution order.
    pub const ALL: [Self; 5] = [
        Self::GenerateSigningKey,
        Self::GenerateEncryptionKey,
        Self::IssueLeafCertificate,
        Self::AttestIdentity,
        Self::JournalIssuance,
    ];

    /// Backdated validity seconds (`storage.rs:203`: `now - 10`).
    pub const BEFORE_SECONDS: i64 = 10;

    /// Forward validity seconds (`storage.rs:204`: `now + 86400`).
    pub const AFTER_SECONDS: i64 = 86_400;

    /// Both EKUs are enabled only for the demo (`storage.rs:187-190`).
    pub const DEMO_EKUS: [&'static str; 2] = ["clientAuth", "serverAuth"];

    /// The count of steps is fixed by the prototype's `issue` function.
    #[must_use]
    pub const fn step_count() -> usize {
        Self::ALL.len()
    }

    /// Whether an EKU name is one the demo leaf carries.
    #[must_use]
    pub fn is_demo_eku(name: &str) -> bool {
        Self::DEMO_EKUS.iter().any(|eku| *eku == name)
    }
}
