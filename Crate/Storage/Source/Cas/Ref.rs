//! A reference to a CAS artifact, carrying the metadata every caller needs.

use super::artifact::Artifact;
use super::blob::Blob;
use super::digest::Digest;

/// A serialized `cas://sha256/...` reference with audience and size metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ref {
    /// Full CAS URI.
    pub uri: String,
    /// Digest identity of the bytes.
    pub digest: Digest,
    /// Declared media type.
    pub media_type: String,
    /// Byte count committed under the digest.
    pub byte_count: u64,
    /// Authorized audience.
    pub audience: Vec<String>,
}

impl Ref {
    /// Declare a reference.
    #[must_use]
    pub fn of(
        uri: String,
        digest: Digest,
        media_type: String,
        byte_count: u64,
        audience: Vec<String>,
    ) -> Self {
        Self {
            uri,
            digest,
            media_type,
            byte_count,
            audience,
        }
    }

    /// Derive the canonical reference from a blob record.
    #[must_use]
    pub fn from(blob: &Blob) -> Self {
        Self::of(
            blob.identity(),
            blob.digest.clone(),
            blob.media_type.clone(),
            blob.byte_count,
            blob.audience.clone(),
        )
    }

    /// Whether the URI uses the documented CAS scheme.
    #[must_use]
    pub fn is_cas(&self) -> bool {
        self.uri.starts_with(crate::CAS_SCHEME)
    }

    /// Whether this reference resolves to a given artifact.
    #[must_use]
    pub fn resolves(&self, artifact: &Artifact) -> crate::permission::Permission {
        self.digest
            .matches(&artifact.digest)
            .and(self.digest.matches(&artifact.blob.digest))
            .and(match self.is_cas() {
                true => crate::permission::Permission::Allow,
                false => crate::permission::Permission::Deny,
            })
    }

    /// Describe the reference.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{} {} bytes {}", self.uri, self.byte_count, self.media_type)
    }
}
