//! SQLite storage affinities used by the chain, as a closed type.

/// The declared storage class of a column.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// `TEXT`: identifiers, UTC timestamps, digests and enumerated states.
    Text,
    /// `INTEGER`: sequences, counters, fences and byte counts.
    Integer,
    /// `REAL`: a measured value; no measurement is produced by this crate.
    Real,
    /// `BLOB`: opaque bytes; the CAS stores bytes by digest identity.
    Blob,
}

impl Type {
    /// The affinity name as the research schema states it.
    #[must_use]
    pub fn affinity(&self) -> &'static str {
        match self {
            Self::Text => "TEXT",
            Self::Integer => "INTEGER",
            Self::Real => "REAL",
            Self::Blob => "BLOB",
        }
    }

    /// Whether a value of another declared type is stored without coercion.
    #[must_use]
    pub fn admits(&self, other: &Self) -> crate::permission::Permission {
        match (self, other) {
            (Self::Text, Self::Text)
            | (Self::Integer, Self::Integer)
            | (Self::Real, Self::Real)
            | (Self::Blob, Self::Blob) => crate::permission::Permission::Allow,
            _ => crate::permission::Permission::Deny,
        }
    }
}
