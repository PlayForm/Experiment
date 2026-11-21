//! Redaction levels and the ban on thought-process claims.
//!
//! Grounded in `docs/00-system-constitution.md:37-39`,
//! `docs/12-dashboard.md:55-59`, `docs/22-hidden-worker-and-correspondence.md:29-39`
//! and `docs/08-data-and-storage.md:40-48`: an observer may see turn count,
//! duration, ciphertext size and success without knowing the text; the
//! dashboard must not issue a decrypt call on an observer's behalf; no ledger
//! or telemetry may claim knowledge of an unpublished agent's internal
//! reasoning. A named decryptor is a separate audited role, not a dashboard
//! feature (`docs/05-event-correspondence.md:134-141`).

/// The disclosure level applied to one projected field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Redaction {
    /// Counts, statuses and opaque identifiers only.
    MetadataOnly,
    /// Hashed or pseudonymous token, never reversible by an observer.
    HashOnly,
    /// Field is withheld entirely from this view.
    Withheld,
}

impl Redaction {
    /// Whether a named field class may be shown at this level.
    #[must_use]
    pub fn admits(self, class: &str) -> crate::Permission {
        match (self, class) {
            (Self::MetadataOnly, "metadata") => crate::Permission::Allow,
            (Self::HashOnly, "digest") | (Self::HashOnly, "metadata") => crate::Permission::Allow,
            _ => crate::Permission::Deny,
        }
    }

    /// No redaction level discloses private reasoning.
    #[must_use]
    pub const fn reveals_thought(self) -> crate::Permission {
        crate::Permission::Deny
    }

    /// An observer never gets an implicit decrypt capability.
    #[must_use]
    pub const fn decrypts(self) -> crate::Permission {
        crate::Permission::Deny
    }
}
