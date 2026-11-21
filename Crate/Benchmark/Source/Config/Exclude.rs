//! A matrix exclusion key.
//!
//! Grounded in README.md:102 (`main__debug__bounded`) and
//! `prun_lab.py:222-228`, where the key `<ref>__<build>__<feature>` is matched
//! against the generated variant identifier.

use serde::{Deserialize, Serialize};

/// A validated exclusion identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exclude(String);

impl Exclude {
    /// Wrap a raw key without allocating when possible.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// The raw key.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /// Split a well-formed key into its three axis identifiers.
    #[must_use]
    pub fn axes(&self) -> Option<[&str; 3]> {
        match self.0.split("__").collect::<Vec<_>>().as_slice() {
            [reference, build, feature] => Some([*reference, *build, *feature]),
            _ => None,
        }
    }
}
