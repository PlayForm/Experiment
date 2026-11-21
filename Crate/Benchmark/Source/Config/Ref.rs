//! A committed source-snapshot selector.
//!
//! Grounded in `prun-bench.schema.json:88-105` (`refs[]`) and
//! `prun_lab.py:244-247`, which resolves `git_ref` to a commit with
//! `git rev-parse --verify <ref>^{commit}`.

use serde::{Deserialize, Serialize};

/// One source ref in the build matrix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ref {
    /// Stable identifier used in variant IDs and exclude keys.
    pub id: String,
    /// Git revision resolved to a clean, committed snapshot.
    pub git_ref: String,
}
