//! Per-session result retention paths.
//!
//! Grounded in `README.md:59-60` (`.prun-bench/runs/<UTC timestamp>/`) and
//! `prun_lab.py:532-536,137,603-605`: each session keeps metadata.json,
//! config.json, summary.csv and one `<group>--<workload>.json` per workload.
//! Paths are data; nothing is created here.

use std::path::PathBuf;

/// One UTC-stamped benchmark session directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    /// Output root (`.prun-bench`).
    pub out: PathBuf,
    /// UTC timestamp directory name.
    pub stamp: String,
}

impl Session {
    pub const METADATA: &'static str = "metadata.json";
    pub const CONFIG: &'static str = "config.json";

    /// Compose a session from its output root and UTC stamp.
    #[must_use]
    pub fn new(out: impl Into<PathBuf>, stamp: impl Into<String>) -> Self {
        Self {
            out: out.into(),
            stamp: stamp.into(),
        }
    }

    /// Session directory under `runs/`.
    #[must_use]
    pub fn directory(&self) -> PathBuf {
        self.out.join("runs").join(&self.stamp)
    }

    /// Retained build manifest path.
    #[must_use]
    pub fn metadata(&self) -> PathBuf {
        self.directory().join(Self::METADATA)
    }

    /// Retained config path.
    #[must_use]
    pub fn config(&self) -> PathBuf {
        self.directory().join(Self::CONFIG)
    }

    /// CSV summary path.
    #[must_use]
    pub fn summary(&self) -> PathBuf {
        self.directory().join("summary.csv")
    }

    /// Hyperfine JSON path for one group and workload (prun_lab.py:137).
    #[must_use]
    pub fn result(&self, group: &str, workload: &str) -> PathBuf {
        self.directory()
            .join(format!("{group}--{workload}.json"))
    }
}
