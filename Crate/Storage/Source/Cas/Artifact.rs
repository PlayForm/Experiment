//! A committed content-addressed artifact with digest identity.

use super::blob::Blob;
use super::digest::Digest;
use super::grant::Grant;

/// An artifact is its digest plus verified byte metadata; it is never a grant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    /// Declared identity of the bytes.
    pub digest: Digest,
    /// Metadata describing the committed bytes.
    pub blob: Blob,
}

impl Artifact {
    /// Declare an artifact, keeping the digest and blob identity coupled.
    #[must_use]
    pub fn of(digest: Digest, blob: Blob) -> Self {
        Self { digest, blob }
    }

    /// The content address of this artifact.
    #[must_use]
    pub fn identity(&self) -> String {
        self.digest.uri()
    }

    /// Whether the declared identity and the blob digest agree and are valid.
    #[must_use]
    pub fn verify(&self) -> crate::permission::Permission {
        let shape = match self.digest.is_valid() {
            true => crate::permission::Permission::Allow,
            false => crate::permission::Permission::Deny,
        };
        self.digest.matches(&self.blob.digest).and(shape)
    }

    /// Whether a grant authorizes a recipient to use this artifact.
    #[must_use]
    pub fn admits(&self, grant: &Grant) -> crate::permission::Permission {
        grant.admits(&self.digest)
    }

    /// Collect all store findings for this artifact.
    #[must_use]
    pub fn findings(&self) -> Vec<crate::warning::Warning> {
        self.blob
            .validate()
            .into_iter()
            .chain(
                match self.digest.matches(&self.blob.digest) {
                    crate::permission::Permission::Allow => None,
                    crate::permission::Permission::Deny => {
                        Some(crate::warning::Warning::Unverified {
                            subject: self.identity(),
                        })
                    }
                },
            )
            .collect()
    }
}
