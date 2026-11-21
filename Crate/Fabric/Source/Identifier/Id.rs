//! A typed canonical identifier value with byte-level validation.
//!
//! The byte parser mirrors `validate_ascii_id` in
//! `rust/lab-identifier/src/lib.rs:23-33` and the Python cross-component mirror
//! in `tests/validate.py:76-104`. It rejects the trailing-newline hole of the
//! original anchored regex (`docs/03:63-71`, `tests/validate.py:57-73`) and
//! never interprets a valid label as a filesystem path (`docs/03:65-71`).

use crate::verdict::Verdict;
use super::kind::Kind;

/// One typed identifier with an immutable semantic kind and revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ID {
    /// Private semantic namespace of this identifier.
    pub kind: Kind,
    /// Canonical logical label, privately stored.
    pub value: String,
    /// Immutable authored revision; reserved space for version and epoch.
    pub revision: u64,
}

impl ID {
    /// Construct an identifier from trusted, already canonical fields.
    #[must_use]
    pub fn of(kind: Kind, value: &str, revision: u64) -> Self {
        Self {
            kind,
            value: value.to_owned(),
            revision,
        }
    }

    /// Validate the byte shape: 1..=128 ASCII bytes, alphanumeric first byte,
    /// alphanumeric or `_.:/-` continuation, no trailing newline, no NUL.
    #[must_use]
    pub fn shaped(&self) -> Verdict {
        let bytes = self.value.as_bytes();
        Verdict::of(
            (1..=crate::identifier::factory::MAX_BYTES).contains(&bytes.len())
                && bytes[0..1]
                    .first()
                    .is_some_and(|first| first.is_ascii_alphanumeric())
                && bytes[1..]
                    .iter()
                    .all(|byte| byte.is_ascii_alphanumeric() || b"_.:/-".contains(byte)),
        )
    }

    /// A canonical identifier is never accepted as a path, even when it is a
    /// valid label such as `A/../` (`docs/03:68-71`).
    #[must_use]
    pub const fn as_path(&self) -> Verdict {
        Verdict::Deny
    }

    /// Whether two identifiers share a namespace slot with this one.
    #[must_use]
    pub fn same_slot(&self, other: &Self) -> Verdict {
        Verdict::of(self.kind == other.kind && self.value == other.value)
    }

    /// The private wire spelling used only inside the registry.
    #[must_use]
    pub fn private_label(&self) -> String {
        format!("{}:{}:{}", self.kind.label(), self.value, self.revision)
    }
}
