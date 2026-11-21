//! Bytes held in the CAS, described by digest identity and metadata.

use super::digest::Digest;
use super::representation::Representation;
use super::storage_class::StorageClass;

/// Metadata for one content-addressed byte stream; bytes are never loaded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blob {
    /// Digest identity of the bytes.
    pub digest: Digest,
    /// Declared media type.
    pub media_type: String,
    /// Number of bytes committed.
    pub byte_count: u64,
    /// Creation time in UTC.
    pub created_utc: String,
    /// Authorized audience for an encrypted envelope.
    pub audience: Vec<String>,
    /// Disclosure class.
    pub storage_class: StorageClass,
    /// Hashing and encryption order.
    pub representation: Representation,
}

impl Blob {
    /// Declare a blob record.
    #[must_use]
    pub fn of(
        digest: Digest,
        media_type: &str,
        byte_count: u64,
        created_utc: &str,
        audience: Vec<String>,
        storage_class: StorageClass,
        representation: Representation,
    ) -> Self {
        Self {
            digest,
            media_type: media_type.to_owned(),
            byte_count,
            created_utc: created_utc.to_owned(),
            audience,
            storage_class,
            representation,
        }
    }

    /// The canonical CAS identity URI.
    #[must_use]
    pub fn identity(&self) -> String {
        self.digest.uri()
    }

    /// Report missing metadata rather than panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        let digest_warning = (!self.digest.is_valid()).then(|| {
            crate::warning::Warning::DigestShape {
                field: "blob digest",
                detail: self.digest.describe(),
            }
        });
        let empty = [
            ("media type", self.media_type.is_empty()),
            ("created utc", self.created_utc.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| crate::warning::Warning::Malformed {
                domain: field,
                detail: "a committed blob must carry this metadata".to_owned(),
            })
        });
        let restricted_without_audience = match self.storage_class {
            StorageClass::Public => None,
            StorageClass::Sealed | StorageClass::Private | StorageClass::Internal => {
                Some(crate::warning::Warning::Withheld { subject: "blob audience" })
            }
        };
        let missing_audience = self.audience.is_empty().then(|| {
            crate::warning::Warning::Malformed {
                domain: "audience",
                detail: "an encrypted artifact needs a named recipient".to_owned(),
            }
        });
        digest_warning
            .into_iter()
            .chain(empty)
            .chain(restricted_without_audience)
            .chain(missing_audience)
            .collect()
    }
}
