//! The ten custody states and their guarded edges.
//!
//! Mirrors `policy/v61-state-machines.yaml:3-12` and its transition list at
//! lines 31-58. `Reserved` and `Staged` are the two phases; `Quarantined`,
//! `Denied` and `Expired` are honest non-success outcomes.

use crate::permission::Permission;

/// One state in a two-phase custody transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    /// A transfer was asked for; nothing was reserved.
    Requested,
    /// The broker is evaluating policy, rights and capacity.
    Preflight,
    /// Bytes and lease were reserved under a one-use nonce.
    Reserved,
    /// Encrypted content was placed in quarantine.
    Staged,
    /// The receiving supervisor verified size, hashes and identity.
    Verified,
    /// The receipt and inventory were committed atomically.
    Committed,
    /// Bytes were fenced and excluded after a crash or timeout.
    Quarantined,
    /// Policy denied the transfer; only trusted audit retains the cause.
    Denied,
    /// The lease or attestation expired before commit.
    Expired,
    /// The task ended and the key was revoked or crypto-shredded.
    Released,
}

impl Phase {
    /// Whether a named edge is permitted by the signed state machine.
    #[must_use]
    pub fn permit(self, to: Self) -> Permission {
        match (self, to) {
            (Self::Requested, Self::Preflight)
            | (Self::Preflight, Self::Reserved)
            | (Self::Preflight, Self::Denied)
            | (Self::Reserved, Self::Staged)
            | (Self::Reserved, Self::Quarantined)
            | (Self::Staged, Self::Verified)
            | (Self::Staged, Self::Quarantined)
            | (Self::Verified, Self::Committed)
            | (Self::Committed, Self::Released) => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Fence an in-flight phase on crash, timeout or stale policy epoch.
    #[must_use]
    pub fn fence(self) -> Self {
        match self {
            Self::Reserved | Self::Staged => Self::Quarantined,
            Self::Requested | Self::Preflight => Self::Denied,
            Self::Verified | Self::Committed | Self::Quarantined | Self::Denied
            | Self::Expired | Self::Released => self,
        }
    }

    /// Whether this phase grants an approved read-only plaintext mount.
    /// Release revokes the key and unmounts, so it grants nothing.
    #[must_use]
    pub fn exposes_plaintext(&self) -> bool {
        match self {
            Self::Committed => true,
            Self::Requested
            | Self::Preflight
            | Self::Reserved
            | Self::Staged
            | Self::Verified
            | Self::Quarantined
            | Self::Denied
            | Self::Expired
            | Self::Released => false,
        }
    }

    /// The wire spelling used by the custody state machine.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Preflight => "preflight",
            Self::Reserved => "reserved",
            Self::Staged => "staged",
            Self::Verified => "verified",
            Self::Committed => "committed",
            Self::Quarantined => "quarantined",
            Self::Denied => "denied",
            Self::Expired => "expired",
            Self::Released => "released",
        }
    }
}
