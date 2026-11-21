//! Independent key rotation with fresh nonces and replay fencing.
//!
//! `docs/04:29` requires unique nonces and tests for replay, rotation, expiry
//! and decryption under a wrong context. `docs/06:43-44` requires power-loss
//! tests during key rotation. A rotation is a typed transition; no key changes
//! here.

use crate::verdict::Verdict;

/// One recorded key-rotation transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rotation {
    /// Authority whose handle rotates.
    pub authority: super::hierarchy::Hierarchy,
    /// Epoch before the rotation.
    pub from_epoch: u64,
    /// Epoch after the rotation.
    pub to_epoch: u64,
    /// One-use nonce bound to this rotation.
    pub nonce: String,
    /// Policy digest active at the rotation.
    pub policy_digest: String,
}

impl Rotation {
    /// Record a rotation from trusted fields.
    #[must_use]
    pub fn of(
        authority: super::hierarchy::Hierarchy,
        from_epoch: u64,
        to_epoch: u64,
        nonce: &str,
        policy: &str,
    ) -> Self {
        Self {
            authority,
            from_epoch,
            to_epoch,
            nonce: nonce.to_owned(),
            policy_digest: policy.to_owned(),
        }
    }

    /// Whether the rotation advances monotonically and carries a fresh nonce.
    #[must_use]
    pub fn valid(&self) -> Verdict {
        Verdict::of(self.to_epoch > self.from_epoch)
            .and(Verdict::filled(&self.nonce))
            .and(Verdict::digest(&self.policy_digest))
    }

    /// Reject a nonce already recorded, including after a reboot
    /// (`docs/09:34-36`).
    #[must_use]
    pub fn fresh(&self, seen: &str) -> Verdict {
        Verdict::of(seen != self.nonce)
    }
}
