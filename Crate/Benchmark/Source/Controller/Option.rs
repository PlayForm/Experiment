//! Global CLI options shared by every subcommand.
//!
//! Grounded in `prun_lab.py:648-660`: `--repo`, `--config`, `--out`, `--jobs`
//! and `--parallel-runners` are parsed before the subcommand and resolved to
//! absolute paths in `main` (667-668).
//!
//! The type name `Option` is mandated by the card; std's option type is never
//! used unqualified inside this file.

use std::path::PathBuf;

use super::{Jobs, ParallelRunners};

/// Resolved controller options.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Option {
    /// Local checkout root.
    pub repo: PathBuf,
    /// Config file path.
    pub config: PathBuf,
    /// Artifact and result root.
    pub out: PathBuf,
    /// Builder concurrency.
    pub jobs: Jobs,
    /// Runner group concurrency.
    pub parallel: ParallelRunners,
}

impl Option {
    /// Defaults matching the argparse declarations (prun_lab.py:648-660).
    #[must_use]
    pub fn baseline() -> Self {
        Self {
            repo: PathBuf::from("."),
            config: PathBuf::from("prun-bench.json"),
            out: PathBuf::from(".prun-bench"),
            jobs: Jobs::DEFAULT,
            parallel: ParallelRunners::Serial,
        }
    }
}
