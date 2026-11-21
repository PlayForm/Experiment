//! The message-signing authority and envelope binding.
//!
//! `docs/04:3` gives it a distinct authority; `docs/04:4` binds HPKE
//! encryption to message version, sender, recipient, purpose and schema
//! epoch; `docs/10:5` repeats the binding. `docs/08:41-42` states a signed
//! producer receipt accompanies each journal append. Nothing signs here.

use crate::verdict::Verdict;
use super::hierarchy::Hierarchy;

/// One message-signing handle bound to a canonical envelope context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    /// Opaque signing handle; no key bytes are present.
    pub handle: String,
    /// Key epoch in which the handle is current.
    pub epoch: u64,
    /// Digest of the current policy version.
    pub policy_digest: String,
    /// Canonical context this signature would bind.
    pub context_digest: String,
}

impl Message {
    /// Record a signing handle from trusted fields.
    #[must_use]
    pub fn of(handle: &str, epoch: u64, policy: &str, context: &str) -> Self {
        Self {
            handle: handle.to_owned(),
            epoch,
            policy_digest: policy.to_owned(),
            context_digest: context.to_owned(),
        }
    }

    /// Whether the envelope context is complete for a signature decision.
    #[must_use]
    pub fn binding_complete(&self) -> Verdict {
        Verdict::filled(&self.handle)
            .and(Verdict::digest(&self.policy_digest))
            .and(Verdict::digest(&self.context_digest))
            .and(Verdict::of(self.epoch > 0))
    }

    /// This authority cannot emit an artifact for a different authority.
    #[must_use]
    pub const fn authority() -> Hierarchy {
        Hierarchy::MessageSigner
    }
}
