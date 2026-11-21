//! Typed descriptor for a signed event or request.
//!
//! `schemas/envelope.schema.json:63-71` carries `sender_signature_b64` and
//! `signature_scope_digest`; `docs/23-interface-inventory.md:75-84` requires
//! signatures over canonical bytes, target, method, idempotency key, credential
//! ID and nonce, with durable nonce replay prevention. This crate records the
//! fields and the denial of truth authority; it never creates or verifies a
//! signature (`Crate/Trust` owns the v4 signed-claim bridge model).

/// A signature descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Signature {
    /// Signature scheme name recorded on the wire.
    pub scheme: &'static str,
    /// Fields bound by the signature.
    pub covers: &'static [&'static str],
    /// Whether a valid signature proves the signed claim is true.
    pub proves_truth: crate::Permission,
}

impl Signature {
    /// Build the canonical signature descriptor.
    #[must_use]
    pub const fn of() -> Self {
        Self {
            scheme: "sender_signature_b64",
            covers: crate::event::Envelope::signature_scope(),
            proves_truth: crate::Permission::Deny,
        }
    }

    /// The replay-prevention mechanism recorded for signed requests.
    #[must_use]
    pub const fn replay_guard() -> &'static str {
        "durable nonce table and idempotent event IDs"
    }

    /// The trust-model signature this descriptor names but never exercises.
    #[must_use]
    pub const fn external_model() -> &'static str {
        crate::TRUST_SIGNATURE_PATH
    }
}
