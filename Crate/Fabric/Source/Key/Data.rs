//! Data-encryption key representation: wrapped or plaintext.
//!
//! `docs/04:5` generates DEKs randomly per artifact or tenant and wraps them
//! under separately protected KEKs. `docs/08:7` keeps the operation inside a
//! non-exporting broker. Two structural states replace a boolean field; no
//! key is generated and no plaintext is held.

use crate::verdict::Verdict;

/// One data-encryption key state with its wrapping relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    /// A wrapped key referenced only by a KEK handle.
    Wrapped {
        /// Opaque handle of the protecting key-encryption key.
        kek: String,
        /// Epoch in which the wrapping was performed.
        epoch: u64,
    },
    /// A plaintext state that must never leave the broker.
    Plaintext {
        /// Digest of the context that authorized this transient state.
        context_digest: String,
    },
}

impl Data {
    /// Whether a wrapped key is structurally complete.
    #[must_use]
    pub fn wrapped_well_formed(&self) -> Verdict {
        match self {
            Self::Wrapped { kek, epoch } => {
                Verdict::filled(kek).and(Verdict::of(*epoch > 0))
            }
            Self::Plaintext { .. } => Verdict::Deny,
        }
    }

    /// Whether this state may be observed outside the broker. It never may.
    #[must_use]
    pub const fn observable(&self) -> bool {
        false
    }

    /// Whether the state exposes plaintext to a caller. Only the transient
    /// in-broker state does, and it is not reachable from a worker value.
    #[must_use]
    pub const fn exposes_plaintext(&self) -> bool {
        matches!(self, Self::Plaintext { .. })
    }
}
