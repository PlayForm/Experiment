//! Domain-separated Ed25519 signature contexts.
//!
//! Grounded in `security/src/crypto.rs:26-27` (message-signing and
//! registration-attestation domains), `crypto.rs:75-80` (domain-prefix helper),
//! `crypto.rs:105-107,130-137` (what is signed), `storage.rs:19` (audit domain)
//! and `storage.rs:315-319` (relay-audit domain). This type builds byte
//! prefixes; it does not sign, verify or hold a key.

/// The four signature domains of the v4 prototype.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Signature {
    /// Signed HPKE envelope (`crypto.rs:26`).
    Message,
    /// CA attestation of an identity binding (`crypto.rs:27`).
    Registration,
    /// Issuance-journal record (`storage.rs:19`).
    Audit,
    /// Relay-observation journal record (`storage.rs:316`).
    RelayAudit,
}

impl Signature {
    /// All signature domains.
    pub const ALL: [Self; 4] = [Self::Message, Self::Registration, Self::Audit, Self::RelayAudit];

    /// The domain-separation bytes for this signature kind.
    #[must_use]
    pub const fn domain(self) -> &'static [u8] {
        match self {
            Self::Message => b"prun-lab/v4/agent-message/ed25519\0",
            Self::Registration => b"prun-lab/v4/identity-attestation/ed25519\0",
            Self::Audit => b"prun-lab/v4/ca-audit/ed25519\0",
            Self::RelayAudit => b"prun-lab/v4/relay-audit/ed25519\0",
        }
    }

    /// Prefix a serialized payload with its domain (`crypto.rs:75-80`).
    #[must_use]
    pub fn domain_prefix(self, payload: &[u8]) -> Vec<u8> {
        let domain = self.domain();
        let mut bytes = Vec::with_capacity(domain.len() + payload.len());
        bytes.extend_from_slice(domain);
        bytes.extend_from_slice(payload);
        bytes
    }

    /// Message and registration domains are distinct (`crypto.rs:245-247`).
    #[must_use]
    pub const fn domains_are_separate() -> bool {
        true
    }
}
