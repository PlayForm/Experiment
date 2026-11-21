//! The HPKE suite descriptor used by signed envelopes.
//!
//! Grounded in `security/src/crypto.rs:9-30`: KEM `X25519HkdfSha256`, AEAD
//! `ChaCha20Poly1305`, KDF `HkdfSha256`, the fixed `INFO` context string and
//! base setup mode; `:172-174` (sender seals with the protected header as AAD),
//! `:226-229` (receiver opens with the same AAD), and `:1-2` (recipient-only,
//! with an independent Ed25519 signature). No HPKE operation is performed.

/// The typed HPKE suite and its context binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hpke {
    /// Key encapsulation mechanism (`crypto.rs:17,28`).
    pub kem: &'static str,
    /// Authenticated encryption algorithm (`crypto.rs:15,29`).
    pub aead: &'static str,
    /// Key derivation function (`crypto.rs:16,30`).
    pub kdf: &'static str,
    /// HPKE setup mode (`crypto.rs:173,226`: `Base`).
    pub mode: &'static str,
    /// Domain-separation context (`crypto.rs:25`).
    pub info: &'static [u8],
    /// Additional authenticated data source (`crypto.rs:171,228`).
    pub aad_source: &'static str,
}

impl Hpke {
    /// The suite used by the prototype.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            kem: "X25519HkdfSha256",
            aead: "ChaCha20Poly1305",
            kdf: "HkdfSha256",
            mode: "Base",
            info: b"prun-lab/v4/agent-message/hpke-x25519-hkdfsha256-chacha20poly1305",
            aad_source: "protected-header-json",
        }
    }

    /// The signature is independent of HPKE (`crypto.rs:1-2`): encryption and
    /// signing are separate values, not one authenticated-encryption scheme.
    #[must_use]
    pub const fn signature_is_independent() -> bool {
        true
    }

    /// Whether the relay can open payloads: it holds no recipient key
    /// (`security/README.md:1-9`, `relay.rs:1-4`, `V4_TRUST_BRIDGE.md:35-37`).
    #[must_use]
    pub const fn relay_can_read_payload() -> bool {
        false
    }
}
