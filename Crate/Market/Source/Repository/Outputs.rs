//! Declared artifact path and metric names of an adapter.
//!
//! Grounded in `repository-adapter.v5.schema.json:117-136` (artifact path and
//! metrics array), `REPOSITORIES.md:17,25-27` (artifact extraction, executable
//! entry points, performance metrics and direction) and `:37-40` (no universal
//! metric exists). Outputs are typed declarations; nothing is extracted.

use crate::Warning;

/// The output declaration of an adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outputs {
    /// Path of the produced artifact (`schema:121-124`).
    pub artifact_path: String,
    /// Names of the metrics this repository can produce (`schema:126-133`).
    pub metrics: Vec<String>,
}

impl Outputs {
    /// Validate the artifact path and metric names.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.artifact_path.is_empty(),
            self.metrics.iter().all(|name| !name.is_empty()),
        ) {
            (false, true) => Ok(()),
            (true, _) => Err(Warning::InvalidValue {
                field: "artifact_path",
                detail: "an artifact path is required".to_owned(),
            }),
            (_, false) => Err(Warning::InvalidValue {
                field: "metrics",
                detail: "empty metric name".to_owned(),
            }),
        }
    }
}
