//! An envelope-encrypted CAS reference with tenant-scoped keying.
//!
//! A globally content-addressed digest is not authorization: a worker cannot
//! infer existence from a digest, and a revocation governs future key uses
//! rather than erasing what a model already saw
//! (`docs/19:15-20`, `docs/21:52-55`, `docs/25:12,14`).

use crate::permission::Permission;

/// One sealed ciphertext reference held in a tenant namespace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encrypted {
    /// Envelope CAS digest, opaque to unentitled readers.
    pub cas_digest: String,
    /// Tenant namespace that scopes the index.
    pub tenant_id: String,
    /// Key scope named by the envelope.
    pub key_scope: String,
    /// Digest of the ciphertext bytes.
    pub ciphertext_digest: String,
    /// Verified plaintext digest in an authorized domain.
    pub plaintext_digest: Option<String>,
    /// Retention window in seconds.
    pub retention_seconds: u64,
}

impl Encrypted {
    /// Whether the reference is structurally complete.
    #[must_use]
    pub fn complete(&self) -> Permission {
        Permission::digest(&self.cas_digest)
            .and(Permission::filled(&self.tenant_id))
            .and(Permission::filled(&self.key_scope))
            .and(Permission::digest(&self.ciphertext_digest))
    }

    /// Whether a caller holding a matching key scope may decrypt.
    #[must_use]
    pub fn decryptable_by(&self, tenant: &str, scope: &str) -> Permission {
        self.complete()
            .and(Permission::of(self.tenant_id == tenant && self.key_scope == scope))
    }

    /// Whether a plaintext digest was actually verified; never inferred.
    #[must_use]
    pub fn verified(&self) -> Permission {
        match &self.plaintext_digest {
            None => Permission::Deny,
            Some(value) => Permission::digest(value),
        }
    }
}
