//! The full capability verification formula as a typed fold.
//!
//! `docs/04:33-38` defines:
//! `ValidIssuerSignature && CurrentEpoch && Unexpired && !Revoked &&
//!  VerifiedWorkloadIdentity && ProofOfPossession && AllowedAction &&
//!  AllowedArtifact && AllowedPurpose && AllowedRecipient &&
//!  PolicyVersionCurrent && FreshNonce`.
//! This value holds those conjuncts; no signature is verified and no grant is
//! issued. `docs/04:46-50` states a zero-knowledge proof is not necessary.

use crate::verdict::Verdict;

/// The twelve structural conjuncts of one capability check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cedar {
    /// Issuer signature was checked by a trusted verifier.
    pub issuer_signature: Verdict,
    /// Epoch is current.
    pub current_epoch: Verdict,
    /// Grant has not expired.
    pub unexpired: Verdict,
    /// Grant has not been revoked.
    pub not_revoked: Verdict,
    /// Calling workload identity was verified.
    pub workload_identity: Verdict,
    /// Proof of possession was supplied.
    pub possession: Verdict,
    /// Requested action is allowed.
    pub allowed_action: Verdict,
    /// Requested artifact is allowed.
    pub allowed_artifact: Verdict,
    /// Requested purpose is allowed.
    pub allowed_purpose: Verdict,
    /// Requested recipient is allowed.
    pub allowed_recipient: Verdict,
    /// Policy version is current.
    pub policy_current: Verdict,
    /// Nonce is fresh and non-replayable.
    pub fresh_nonce: Verdict,
}

impl Cedar {
    /// Fold all twelve conjuncts; a single denial fails closed.
    #[must_use]
    pub const fn verify(&self) -> Verdict {
        self.issuer_signature
            .and(self.current_epoch)
            .and(self.unexpired)
            .and(self.not_revoked)
            .and(self.workload_identity)
            .and(self.possession)
            .and(self.allowed_action)
            .and(self.allowed_artifact)
            .and(self.allowed_purpose)
            .and(self.allowed_recipient)
            .and(self.policy_current)
            .and(self.fresh_nonce)
    }

    /// The twelve named conjuncts in formula order.
    #[must_use]
    pub const fn conjuncts() -> [&'static str; 12] {
        [
            "issuer_signature",
            "current_epoch",
            "unexpired",
            "not_revoked",
            "workload_identity",
            "possession",
            "allowed_action",
            "allowed_artifact",
            "allowed_purpose",
            "allowed_recipient",
            "policy_current",
            "fresh_nonce",
        ]
    }

    /// A grant is authorization, never the secret content it protects
    /// (`docs/04:40-41`).
    #[must_use]
    pub const fn grant_is_content() -> bool {
        false
    }
}
