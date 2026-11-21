//! The private scope-to-canonical mapping kept out of telemetry.
//!
//! Mirrors every field of `schemas/private-crosswalk.schema.json`: scope,
//! opaque_handle, canonical_digest, audience, epoch, purpose,
//! mapping_cipher_digest and expiry. The example is synthetic plaintext;
//! a deployed record must be encrypted (`docs/07:16-20`, `docs/03:50-52`).

use crate::verdict::Verdict;

/// One private crosswalk record reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crosswalk {
    /// Scope identifier under which the handle is scoped.
    pub scope: String,
    /// Opaque handle exposed to authorized views.
    pub opaque_handle: String,
    /// Digest of the canonical identity, never exported in plaintext.
    pub canonical_digest: String,
    /// Audience the mapping is minted for.
    pub audience: String,
    /// Key epoch of the mapping.
    pub epoch: u64,
    /// Authorized purpose.
    pub purpose: String,
    /// Digest of the encrypted mapping payload.
    pub mapping_cipher_digest: String,
    /// Canonical expiry instant.
    pub expiry: String,
}

impl Crosswalk {
    /// Record a crosswalk reference from trusted fields.
    #[must_use]
    pub fn of(
        scope: &str,
        handle: &str,
        canonical: &str,
        audience: &str,
        epoch: u64,
        purpose: &str,
        cipher: &str,
        expiry: &str,
    ) -> Self {
        Self {
            scope: scope.to_owned(),
            opaque_handle: handle.to_owned(),
            canonical_digest: canonical.to_owned(),
            audience: audience.to_owned(),
            epoch,
            purpose: purpose.to_owned(),
            mapping_cipher_digest: cipher.to_owned(),
            expiry: expiry.to_owned(),
        }
    }

    /// Whether the crosswalk reference is structurally complete.
    #[must_use]
    pub fn complete(&self) -> Verdict {
        Verdict::filled(&self.scope)
            .and(Verdict::opaque(&self.opaque_handle))
            .and(Verdict::digest(&self.canonical_digest))
            .and(Verdict::filled(&self.audience))
            .and(Verdict::of(self.epoch >= 1))
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::digest(&self.mapping_cipher_digest))
            .and(Verdict::filled(&self.expiry))
    }

    /// Correlation across views requires an authorized resolver and the
    /// private crosswalk (`docs/03:50-52`, `README.md:16-18`).
    #[must_use]
    pub const fn rejoin_without_authorization() -> Verdict {
        Verdict::Deny
    }
}
