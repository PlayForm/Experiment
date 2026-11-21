//! One runner image identity inside build metadata.
//!
//! Grounded in `prun_lab.py:366` (`images[target] = {"tag":...,
//! "image_id":...}`) retained in `runner_images`.

use serde::{Deserialize, Serialize};

/// A tag and resolved image ID pair.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageRecord {
    pub tag: String,
    pub image_id: String,
}
