//! The signed event envelope descriptor.
//!
//! `schemas/envelope.schema.json:7-99` requires 16 fields. The
//! signature binds sender, recipient, nonce, ciphertext digest, idempotency
//! key, scope and receipt; the canonical serialized payload bytes, request
//! target, method, idempotency key, credential ID and nonce are covered
//! (`docs/23-interface-inventory.md:75-80`, `EVENT_INDEX.md:143-146`). A signed
//! envelope does not prove the result is true: the experiment authority must
//! sign the result of actual execution.

use crate::event::Privacy;

/// The signed wire envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Envelope {
    /// Envelope protocol version.
    pub protocol_version: u8,
    /// Visibility class carried on the envelope.
    pub visibility: Privacy,
    /// Required envelope fields in schema order.
    pub fields: &'static [&'static str],
}

impl Envelope {
    /// Declared envelope protocol version.
    pub const VERSION: u8 = 6;

    /// Build the canonical v6 envelope descriptor.
    #[must_use]
    pub const fn of() -> Self {
        Self {
            protocol_version: Self::VERSION,
            visibility: Privacy::MetadataOnly,
            fields: &[
            "envelope_id",
            "protocol_version",
            "message_type",
            "sender_credential_id",
            "recipient_service_id",
            "correlation_id",
            "idempotency_key",
            "created_utc",
            "expires_utc",
            "nonce",
            "ciphertext",
            "ciphertext_digest",
            "sender_signature_b64",
            "signature_scope_digest",
            "visibility",
            "tracing_correlation_digest",
            ],
        }
    }

    /// Inputs bound by the signature.
    #[must_use]
    pub const fn signature_scope() -> &'static [&'static str] {
        &[
            "canonical_payload_bytes",
            "request_target",
            "method",
            "idempotency_key",
            "credential_id",
            "nonce",
        ]
    }

    /// Whether the signature alone proves a scientific result.
    #[must_use]
    pub const fn proves_truth() -> crate::Permission {
        crate::Permission::Deny
    }
}
