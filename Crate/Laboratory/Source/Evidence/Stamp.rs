//! Signed stamps: identity binding, never manufactured truth.
//!
//! Grounded in `docs/09-cryptographic-trust.md:31-49` and
//! `docs/00-system-constitution.md:53-55`: a stamp binds signer scope, subject
//! digest, policy digest, nonce, expiry and candidate/fixture digests; domain
//! separated contexts mean a signature for one purpose is never equivalent to
//! another. A researcher signature attests authorship of opinion, not a
//! measurement, and CA validity is not scientific validity
//! (`docs/02-pitfalls-and-verification.md:19`, `docs/15` BND-019). This type
//! validates a stamp's shape and scope; it signs and verifies nothing.

use crate::{Digest, Permission, Warning};

/// A binding record identifying a signer and the evidence it attests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stamp {
    /// Authorized service role of the signer.
    pub signer: &'static str,
    /// Domain-separated signature context.
    pub context: &'static str,
    /// Digest of the signed subject.
    pub subject: Digest,
    /// Digest of the policy pinned when the stamp was made.
    pub policy: Digest,
    /// Single-use nonce replay protection.
    pub nonce: String,
    /// Whether the stamp was inside its validity window when checked.
    pub current: Permission,
}

impl Stamp {
    /// Build a stamp, rejecting a replayed or empty nonce with a warning.
    #[must_use]
    pub fn of(
        signer: &'static str,
        context: &'static str,
        subject: Digest,
        policy: Digest,
        nonce: String,
        current: Permission,
    ) -> Result<Self, Warning> {
        match nonce.is_empty() {
            true => Err(Warning::Replay {
                message_id: "empty nonce".to_owned(),
            }),
            false => Ok(Self {
                signer,
                context,
                subject,
                policy,
                nonce,
                current,
            }),
        }
    }

    /// Whether this stamp may be treated as evidence at the given rank.
    #[must_use]
    pub fn binds(&self, subject: &Digest, context: &'static str) -> Permission {
        match (
            &self.subject == subject,
            self.context == context,
            self.current,
        ) {
            (true, true, Permission::Allow) => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// A stamp never proves the truth of what was signed.
    #[must_use]
    pub const fn proves_truth(&self) -> Permission {
        Permission::Deny
    }
}
