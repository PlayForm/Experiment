//! A content-addressed artifact extracted by an adapter.
//!
//! Grounded in `repository-adapter.v5.schema.json:117-136` (artifact path and
//! metric names), `REPOSITORIES.md:17,25-27` (artifact extraction, entry points
//! and metric direction), `README.md:57-58` (a hash proves attribution and
//! integrity of bytes, not truth) and `SYSTEM.md:127-129` (immutable CAS
//! outputs). Nothing is extracted or hashed here.

use crate::{Digest, Warning};

use super::Direction;

/// One extracted, immutable artifact declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    /// Declared output path (`schema:121-124`).
    pub path: String,
    /// Content address of the bytes (`schema:157-160` style).
    pub sha256: Digest,
    /// Metric names this artifact reports (`schema:126-133`).
    pub metrics: Vec<String>,
    /// Direction of the primary metric (`REPOSITORIES.md:25`).
    pub direction: Direction,
}

impl Artifact {
    /// Validate the path and metric declarations.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.path.is_empty(),
            self.metrics.iter().all(|name| !name.is_empty()),
        ) {
            (false, true) => Ok(()),
            (true, _) => Err(Warning::MissingArtifact {
                digest: "artifact path".to_owned(),
            }),
            (_, false) => Err(Warning::InvalidValue {
                field: "metrics",
                detail: "empty metric name".to_owned(),
            }),
        }
    }

    /// A digest proves byte integrity, not that the artifact is a correct or
    /// faster implementation (`README.md:57-58`).
    #[must_use]
    pub const fn digest_proves_correctness() -> crate::Permission {
        crate::Permission::Deny
    }
}
