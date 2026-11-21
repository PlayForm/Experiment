//! Provenance of a sanitized derivative.
//!
//! Grounded in `SYSTEM.md:67-70` (a distinct signed, sanitized derivative with
//! provenance links, possibly private, to its parents), `PUBLISHING.md:59-61`
//! (a private artifact can stay access-controlled while the public citation is
//! a salted or content-addressed commitment; a hash alone cannot establish the
//! truth of an unrevealed claim) and `V4_TRUST_BRIDGE.md:43-44` (the public paper
//! hash is content-addressed, the private link lives in an access-controlled
//! repository-specific ledger). No link is opened here.

use crate::{Digest, Permission, Warning};

/// The public/private provenance split of a released paper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provenance {
    /// Content-addressed public paper hash (`V4_TRUST_BRIDGE.md:43`).
    pub public_hash: Digest,
    /// Private parent links held in an access-controlled ledger (`SYSTEM.md:69`).
    pub private_parent_links: Vec<String>,
}

impl Provenance {
    /// Construct a provenance record.
    #[must_use]
    pub const fn of(public_hash: Digest) -> Self {
        Self {
            public_hash,
            private_parent_links: Vec::new(),
        }
    }

    /// Validate that no private link was written into the public record.
    pub fn validate(&self) -> Result<(), Warning> {
        match self.private_parent_links.iter().all(|link| !link.is_empty()) {
            true => Ok(()),
            false => Err(Warning::InvalidValue {
                field: "private_parent_links",
                detail: "empty parent link".to_owned(),
            }),
        }
    }

    /// A public citation may be a hash, but the private artifact stays private
    /// (`PUBLISHING.md:59-61`).
    #[must_use]
    pub const fn may_expose_private_link() -> Permission {
        Permission::Deny
    }

    /// A hash alone proves integrity of bytes, not truth of the claim
    /// (`README.md:57-58`, `V4_TRUST_BRIDGE.md:57-63`).
    #[must_use]
    pub const fn hash_proves_truth() -> Permission {
        Permission::Deny
    }
}
