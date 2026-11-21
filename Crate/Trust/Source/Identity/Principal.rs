//! Separated principals for the research economy.
//!
//! Grounded in the card's separated set (coordinator, oracle, runner,
//! publication) and `V4_TRUST_BRIDGE.md:11-31`: each sender role carries its
//! own message kind, authorized recipient and required evidence, and
//! "authenticated identity alone does not authorize funding, publication or
//! release". `V4_TRUST_BRIDGE.md:57-63` adds that a CA-signed principal is not
//! a scientific-validity stamp. No principal here can sign anything.

use crate::permission::Permission;

/// A separated principal in the trust and research economy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Principal {
    /// Deterministic work-order owner (`V4_TRUST_BRIDGE.md:19`).
    Coordinator,
    /// Independent correctness oracle (`V4_TRUST_BRIDGE.md:23`).
    Oracle,
    /// Exclusive measurement runner.
    Runner,
    /// Scoped publication custodian (`V4_TRUST_BRIDGE.md:24-25`).
    Publication,
    /// Bid and claim submitter (`V4_TRUST_BRIDGE.md:20-21`).
    Researcher,
    /// Signed reservation receipt issuer (`V4_TRUST_BRIDGE.md:22`).
    Treasurer,
    /// Publisher of sanitized papers (`V4_TRUST_BRIDGE.md:25`).
    Publisher,
    /// Proposal-only integration researcher (`V4_TRUST_BRIDGE.md:26`).
    Integrator,
}

impl Principal {
    /// The four principals named by the card as separated.
    pub const CORE: [Self; 4] = [Self::Coordinator, Self::Oracle, Self::Runner, Self::Publication];

    /// All eight principals named by the v5 role-specific message table.
    pub const ALL: [Self; 8] = [
        Self::Coordinator,
        Self::Oracle,
        Self::Runner,
        Self::Publication,
        Self::Researcher,
        Self::Treasurer,
        Self::Publisher,
        Self::Integrator,
    ];

    /// Whether this principal may sign a measurement. No reasoning-edge
    /// principal may; only the deterministic oracle and runner may.
    #[must_use]
    pub const fn may_sign_measurement(self) -> Permission {
        match self {
            Self::Oracle | Self::Runner => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// No principal may change active policy by identity alone; policy is a
    /// deterministic engine decision (`V4_TRUST_BRIDGE.md:49-63`).
    #[must_use]
    pub const fn may_change_policy() -> Permission {
        Permission::Deny
    }

    /// Identity plus role never authorizes funding, publication or release
    /// without the payload attestations (`V4_TRUST_BRIDGE.md:28-31`).
    #[must_use]
    pub const fn identity_alone_authorizes() -> Permission {
        Permission::Deny
    }

    /// Whether the principal belongs to the card's separated core set.
    #[must_use]
    pub fn is_core(self) -> bool {
        Self::CORE.contains(&self)
    }
}
