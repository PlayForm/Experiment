//! A validated SHA-256 digest value.
//!
//! Grounded in every `CHECK (length(...) = 64)` clause of `sql/lineage.sql`
//! (`:11, :17`) and the content-addressed artifact path
//! `objects/sha256/<first-two>/<digest>` (`REQUIREMENTS.md:728-729`). A digest
//! is evidence, never a generated claim; malformed text is rejected with a
//! warning-friendly `Fault`, never a panic.

use serde::{Deserialize, Serialize};

/// A lowercase, 64-character hex SHA-256 digest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Digest(String);

impl Digest {
    /// Validate and wrap a digest string.
    pub fn parse(value: &str) -> Result<Self, crate::Fault> {
        let lowercase_hex =
            value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
        match value.len() == 64 && lowercase_hex {
            true => Ok(Self(value.to_owned())),
            false => Err(crate::Fault::Conflicting {
                domain: "digest",
                detail: "expected 64 lowercase hexadecimal characters".to_owned(),
            }),
        }
    }

    /// The full digest text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The first two characters used by the CAS layout.
    #[must_use]
    pub fn prefix(&self) -> &str {
        self.0.get(0..2).unwrap_or("??")
    }

    /// A short form for human-readable reports.
    #[must_use]
    pub fn short(&self) -> &str {
        self.0.get(0..12).unwrap_or(&self.0)
    }
}

impl std::fmt::Display for Digest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
