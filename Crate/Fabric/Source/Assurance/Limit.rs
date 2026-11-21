//! Explicit limitations that must travel with every claim.
//!
//! `README.md:3-6` states there is no audited cryptography and no enclave.
//! `docs/01:56-58` rejects "completely non-hackable". `docs/04:52-60` limits
//! what end-to-end encryption and secret-lifetime hygiene can protect.
//! `docs/05:52-62` warns that binaries can leak private names. A declared
//! limitation is an honest typed value, not an admission of a test result.

use crate::verdict::Verdict;

/// One explicit limitation of the design.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Limit {
    /// No independent cryptographic or security audit was performed.
    NoAuditedCryptography,
    /// No enclave or confidential-computing attestation is claimed.
    NoEnclave,
    /// No real egress, mount or host-isolation test was run.
    NoOperationalProof,
    /// A privileged host owner can inspect an ordinary Docker host.
    HostOwnerCanInspect,
    /// Side channels prevent absolute traffic secrecy.
    NoAbsoluteTrafficSecrecy,
    /// One genesis commitment is not a universal seed.
    NoUniversalSeed,
    /// A JSON Schema cannot prove a cryptographic property.
    SchemaCannotProveCrypto,
    /// Compiled binaries may embed private names as plaintext.
    BinaryMayLeakNames,
    /// Revocation cannot recall plaintext already consumed.
    RevocationCannotRecall,
}

impl Limit {
    /// All limitations declared by the assurance plan.
    #[must_use]
    pub const fn all() -> [Self; 9] {
        [
            Self::NoAuditedCryptography,
            Self::NoEnclave,
            Self::NoOperationalProof,
            Self::HostOwnerCanInspect,
            Self::NoAbsoluteTrafficSecrecy,
            Self::NoUniversalSeed,
            Self::SchemaCannotProveCrypto,
            Self::BinaryMayLeakNames,
            Self::RevocationCannotRecall,
        ]
    }

    /// A limitation must be disclosed, never suppressed.
    #[must_use]
    pub const fn disclosed(&self) -> Verdict {
        Verdict::Allow
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::NoAuditedCryptography => "no-audited-cryptography",
            Self::NoEnclave => "no-enclave",
            Self::NoOperationalProof => "no-operational-proof",
            Self::HostOwnerCanInspect => "host-owner-can-inspect",
            Self::NoAbsoluteTrafficSecrecy => "no-absolute-traffic-secrecy",
            Self::NoUniversalSeed => "no-universal-seed",
            Self::SchemaCannotProveCrypto => "schema-cannot-prove-crypto",
            Self::BinaryMayLeakNames => "binary-may-leak-names",
            Self::RevocationCannotRecall => "revocation-cannot-recall",
        }
    }
}
