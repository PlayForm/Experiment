//! A signed commitment that attests state but is never a master key.
//!
//! `docs/06:27-30` permits publishing a signed genesis commitment as an audit
//! anchor; `docs/01:41-48` forbids treating it as an encryption seed. The same
//! shape underlies the audit and migration digest chain. No hash is computed.

use crate::verdict::Verdict;

/// One commitment to a canonical, signed state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Commitment {
    /// Digest of the committed canonical bytes.
    pub digest: String,
    /// Epoch at which the commitment was recorded.
    pub epoch: u64,
    /// Kind of state attested by this commitment.
    pub kind: &'static str,
}

impl Commitment {
    /// Record a commitment from trusted fields.
    #[must_use]
    pub fn of(digest: &str, epoch: u64, kind: &'static str) -> Self {
        Self {
            digest: digest.to_owned(),
            epoch,
            kind,
        }
    }

    /// Whether the commitment has a valid digest and nonzero epoch.
    #[must_use]
    pub fn intact(&self) -> Verdict {
        Verdict::digest(&self.digest)
            .and(Verdict::of(self.epoch > 0))
            .and(Verdict::filled(self.kind))
    }

    /// A commitment attests; it never encrypts or derives a data key.
    #[must_use]
    pub const fn as_encryption_seed() -> Verdict {
        Verdict::Deny
    }
}
