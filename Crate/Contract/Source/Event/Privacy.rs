//! Privacy classification carried on the signed event wire message.
//!
//! `events/catalog.json` assigns `metadata-only` to the 57 canonical kinds and
//! `restricted-metadata` to the 30 v6.1 kinds. The signed envelope itself is
//! metadata-only; content lives in encrypted CAS and is reached by policy
//! (`api/events.asyncapi.yaml:5-7`, `EVENT_INDEX.md:3-4`).

use crate::Permission;

/// Visibility of an event record on the wire.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Privacy {
    /// Authorized dashboard metadata only.
    MetadataOnly,
    /// Restricted metadata for trusted software subscribers only.
    RestrictedMetadata,
}

impl Privacy {
    /// Wire spelling from `events/catalog.json`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata-only",
            Self::RestrictedMetadata => "restricted-metadata",
        }
    }

    /// Whether an observer projection may show this record.
    #[must_use]
    pub const fn observer_access(self) -> Permission {
        match self {
            Self::MetadataOnly => Permission::Allow,
            Self::RestrictedMetadata => Permission::Deny,
        }
    }

    /// Decode a wire privacy value.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "metadata-only" => Some(Self::MetadataOnly),
            "restricted-metadata" => Some(Self::RestrictedMetadata),
            _ => None,
        }
    }
}
