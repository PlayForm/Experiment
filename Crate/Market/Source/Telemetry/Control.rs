//! Control and cryptographic plane attributes.
//!
//! Grounded in `TELEMETRY.md:5-8`: task lease, CA issuance, recipient
//! identities, certificate rotation, envelopes accepted/denied, coordinator
//! scheduling, status transitions and ledger commits. No plaintext payload or
//! private prompt is permitted in an attribute.

/// The control/cryptographic telemetry plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Control;

impl Control {
    /// The declared control-plane attribute set (`TELEMETRY.md:5-8`).
    pub const ATTRIBUTES: [&'static str; 8] = [
        "task_lease",
        "ca_issuance",
        "recipient_identity",
        "certificate_rotation",
        "envelope_accepted_denied",
        "coordinator_scheduling",
        "status_transition",
        "ledger_commit",
    ];

    /// The plane this value belongs to.
    #[must_use]
    pub const fn plane() -> super::Plane {
        super::Plane::Control
    }

    /// Control attributes never include decrypted research content
    /// (`TELEMETRY.md:7-8`).
    #[must_use]
    pub const fn may_include_payload() -> crate::Permission {
        crate::Permission::Deny
    }
}
