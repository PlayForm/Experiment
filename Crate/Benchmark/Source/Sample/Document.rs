//! A complete Hyperfine schema-v2 JSON export.
//!
//! Grounded in `prun_lab.py:606-612` (read the result file, check the schema
//! version, take the first entry as baseline). Malformed JSON is a typed
//! `Fault`; version drift is a `Warning` with a V2 interpretation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Fault, Warning};

use super::{Sample, SchemaVersion};

/// One Hyperfine JSON document.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Document {
    #[serde(default = "SchemaVersion::number")]
    pub schema_version: u64,
    #[serde(default)]
    pub results: Vec<Sample>,
}

impl Document {
    /// Decode a result file, accumulating warnings for version drift.
    pub fn decode(text: &str, warnings: &mut Vec<Warning>) -> Result<Self, Fault> {
        let value: Value = serde_json::from_str(text).map_err(|error| Fault::JSON {
            path: "hyperfine".to_owned(),
            detail: error.to_string(),
        })?;
        let found = value
            .get("schema_version")
            .and_then(Value::as_u64)
            .unwrap_or(SchemaVersion::NUMBER);
        let _ = SchemaVersion::accept(found, warnings);
        serde_json::from_value(value).map_err(|error| Fault::JSON {
            path: "hyperfine".to_owned(),
            detail: error.to_string(),
        })
    }

    /// The first entry's mean is the CSV baseline (prun_lab.py:612).
    #[must_use]
    pub fn baseline(&self) -> Option<f64> {
        self.results.first().map(Sample::mean)
    }
}
