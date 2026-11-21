//! A content hash, injected as evidence and never fabricated here.
//!
//! Grounded in `RESEARCH_GATE.md:53-68` (binary digest and exact build key,
//! separate content hashes) and `STORAGE.md:7-20` (SHA-256 CAS, read-only once
//! published). This crate does not hash bytes: a `Digest` is an observed 64-hex
//! value validated at its boundary, so a missing or malformed hash degrades to
//! a warning instead of panicking.

use crate::warning::Warning;

/// A validated 64-character lowercase hex content hash.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Digest {
    /// The hex digest value.
    pub value: String,
}

impl Digest {
    /// Required hex length for a SHA-256 digest.
    pub const LEN: usize = 64;

    /// Validate an observed digest, rejecting malformed input gracefully.
    pub fn parse(value: String) -> Result<Self, Warning> {
        let well_formed = value.len() == Self::LEN
            && value.bytes().all(|byte| byte.is_ascii_hexdigit());
        match well_formed {
            true => Ok(Self { value }),
            false => Err(Warning::MissingArtifact { digest: value }),
        }
    }

    /// The digest as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Whether two digests are the same immutable artifact.
    #[must_use]
    pub fn same(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
