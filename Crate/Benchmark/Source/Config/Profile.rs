//! The Cargo profile a build config compiles against.
//!
//! Grounded in `prun-bench.schema.json:118` (`enum: ["release","dev"]`) and
//! `prun_lab.py:203-207`, which rejects any other profile.

use serde::{Deserialize, Serialize};

/// Supported Cargo profiles (schema.json:118; prun_lab.py:204).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    Release,
    Dev,
}

impl Profile {
    /// Cargo's `--profile` value (prun_lab.py:315).
    #[must_use]
    pub const fn cargo_name(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Dev => "dev",
        }
    }

    /// Cargo's output directory for this profile (prun_lab.py:313).
    #[must_use]
    pub const fn outdir(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Dev => "debug",
        }
    }

    /// The runner group this profile is routed to (prun_lab.py:236).
    #[must_use]
    pub const fn group(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Dev => "diagnostic",
        }
    }
}
