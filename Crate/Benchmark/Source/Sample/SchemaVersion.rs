//! The supported Hyperfine JSON export schema version.
//!
//! Grounded in `README.md:132-134` (Hyperfine 2.x exports schema version 2)
//! and `prun_lab.py:608-611`, which rejects any other version. This model warns
//! and degrades rather than aborting.

use crate::Warning;

/// Accepted Hyperfine export schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaVersion {
    V2,
}

impl SchemaVersion {
    /// The integer carried in Hyperfine's `schema_version` field.
    pub const NUMBER: u64 = 2;

    /// Serde default for the document's version field.
    #[must_use]
    pub const fn number() -> u64 {
        Self::NUMBER
    }

    /// Accept a version, warning and degrading to V2 on mismatch.
    #[must_use]
    pub fn accept(found: u64, warnings: &mut Vec<Warning>) -> Self {
        let _ = (found != Self::NUMBER).then(|| {
            warnings.push(Warning::Version {
                found: found.to_string(),
                expected: Self::NUMBER.to_string(),
            })
        });
        Self::V2
    }
}
