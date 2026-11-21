//! SHA-256 digest values and the hashing step as data.
//!
//! Grounded in `prun_lab.py::digest` (177-180) and its truncations:
//! `[:20]` for lock keys and builder tags (:279, :380), `[:16]` for fixture
//! volumes and artifact fingerprints (:377, :466). Computing a digest is an
//! executor concern; this type carries the value and models the invocation.

use serde::{Deserialize, Serialize};

/// A lowercase hexadecimal SHA-256 value.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Digest(String);

impl Digest {
    /// Wrap a hex digest supplied by an executor.
    #[must_use]
    pub fn from_hex(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// The full hex string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Truncated fingerprint used in directory and tag names.
    #[must_use]
    pub fn prefix(&self, length: usize) -> &str {
        self.0
            .get(..length.min(self.0.len()))
            .unwrap_or(self.0.as_str())
    }

    /// The hashing invocation modelled as argv (prun_lab.py:177-180).
    #[must_use]
    pub fn command(payload: &str) -> Vec<String> {
        vec!["sha256".to_owned(), payload.to_owned()]
    }
}
