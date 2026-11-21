//! Record classification classes and their visibility.
//!
//! `policy/record-classification.yaml:1-42` declares four classes. Metadata is
//! not automatically anonymous: an observer sees only approved projections,
//! never sealed content (`policy/record-classification.yaml:40-42`).

/// A record classification class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Classification {
    /// Campaign status, aggregate results and sanitized papers.
    PublicMeta,
    /// Service IDs, span IDs and certificate serials.
    InternalMeta,
    /// Patch diffs, private claims and test counterexamples.
    Restricted,
    /// Raw conversations, tool transcripts, holdouts and private keys.
    Sealed,
}

impl Classification {
    /// Wire class name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicMeta => "public-meta",
            Self::InternalMeta => "internal-meta",
            Self::Restricted => "restricted",
            Self::Sealed => "sealed",
        }
    }

    /// Who may see this class.
    #[must_use]
    pub const fn visible_to(self) -> &'static str {
        match self {
            Self::PublicMeta => "observer, operator",
            Self::InternalMeta => "operator, authorized-auditor",
            Self::Restricted => "assigned workers via scoped snapshots, trusted validators",
            Self::Sealed => "named decryptor only",
        }
    }

    /// Retention rule for this class.
    #[must_use]
    pub const fn retention(self) -> &'static str {
        match self {
            Self::PublicMeta => "until owner retention policy expires",
            Self::InternalMeta => "operational",
            Self::Restricted => "append-only encrypted",
            Self::Sealed => {
                "encryption and deletion policy; ciphertext may persist beyond key destruction"
            }
        }
    }

    /// Whether an observer projection may include this class.
    #[must_use]
    pub const fn observer_access(self) -> crate::Permission {
        match self {
            Self::PublicMeta => crate::Permission::Allow,
            Self::InternalMeta | Self::Restricted | Self::Sealed => crate::Permission::Deny,
        }
    }
}
