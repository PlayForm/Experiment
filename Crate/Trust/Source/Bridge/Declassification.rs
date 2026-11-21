//! Disclosure classes and the declassification grant.
//!
//! Grounded in `V4_TRUST_BRIDGE.md:33-47`: the relay sees metadata and
//! ciphertext but not HPKE payloads; notebooks are encrypted per researcher;
//! the publisher receives only expressly submitted artifacts; the treasury
//! sees quantities, claim IDs and signed receipts but no hidden reasoning; the
//! public paper hash is content-addressed and no telemetry attribute contains
//! plaintext hypotheses or confidential code. No document is declassified here.

use crate::permission::Permission;

/// A class of information in the v4/v5 disclosure policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Declassification {
    /// Relay-visible transport metadata (`V4_TRUST_BRIDGE.md:35-36`).
    Metadata,
    /// Relay-visible ciphertext (`:36-37`).
    Ciphertext,
    /// An artifact expressly submitted to the publisher (`:38-39`).
    SubmittedArtifact,
    /// A sanitized, content-addressed public paper (`:43-44`).
    SanitizedPaper,
    /// A researcher's private reasoning or notebook (`:37,41,45-47`).
    HiddenReasoning,
}

impl Declassification {
    /// All disclosure classes.
    pub const ALL: [Self; 5] = [
        Self::Metadata,
        Self::Ciphertext,
        Self::SubmittedArtifact,
        Self::SanitizedPaper,
        Self::HiddenReasoning,
    ];

    /// Whether the class is observable by the relay.
    #[must_use]
    pub const fn visible_to_relay(self) -> Permission {
        match self {
            Self::Metadata | Self::Ciphertext => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Whether publication of this class requires an explicit grant
    /// (`:25,42-47`).
    #[must_use]
    pub const fn requires_grant(self) -> Permission {
        match self {
            Self::SanitizedPaper | Self::SubmittedArtifact => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// No class may place a plaintext hypothesis or confidential code in a
    /// telemetry attribute (`:45`).
    #[must_use]
    pub const fn may_enter_telemetry(self) -> Permission {
        Permission::Deny
    }
}
