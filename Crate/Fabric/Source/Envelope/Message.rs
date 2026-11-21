//! The HPKE binding context for one semantically encrypted message.
//!
//! `docs/04:4` binds HPKE to source, purpose, schema epoch and recipient and
//! requires authenticated HPKE mode or a separate signature for author
//! identity. `docs/10:5` repeats message version, sender, recipient, purpose
//! and schema epoch. Two identically shaped envelopes can carry different
//! internal schemas (`docs/02:34-35`). Nothing is sealed or opened here.

use crate::verdict::Verdict;

/// One authenticated binding context for a message envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    /// Wire protocol version.
    pub version: String,
    /// Authenticated sender workload.
    pub sender: String,
    /// Bound recipient endpoint.
    pub recipient: String,
    /// Authorized purpose.
    pub purpose: String,
    /// Schema epoch of the payload.
    pub schema_epoch: u64,
    /// Digest of the canonical context.
    pub context_digest: String,
}

impl Message {
    /// Record a message context from trusted fields.
    #[must_use]
    pub fn of(
        sender: &str,
        recipient: &str,
        purpose: &str,
        schema_epoch: u64,
        context: &str,
    ) -> Self {
        Self {
            version: crate::PROTOCOL_REVISION.to_owned(),
            sender: sender.to_owned(),
            recipient: recipient.to_owned(),
            purpose: purpose.to_owned(),
            schema_epoch,
            context_digest: context.to_owned(),
        }
    }

    /// Whether every authenticated binding field is present.
    #[must_use]
    pub fn binding_complete(&self) -> Verdict {
        Verdict::of(self.version == crate::PROTOCOL_REVISION)
            .and(Verdict::filled(&self.sender))
            .and(Verdict::filled(&self.recipient))
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::of(self.schema_epoch >= 1))
            .and(Verdict::digest(&self.context_digest))
    }

    /// A recipient may not be swapped for another endpoint after sealing
    /// (`docs/09:27-28`).
    #[must_use]
    pub fn bound_to(&self, observed_recipient: &str) -> Verdict {
        Verdict::of(self.recipient == observed_recipient)
    }
}
