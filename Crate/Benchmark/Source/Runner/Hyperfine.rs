//! The pinned Hyperfine invocation emitted inside a runner.
//!
//! Grounded in `README.md:13-14` (both images pin Hyperfine **2.0.0**),
//! `prun_lab.py:96` (the build arg) and `RUNNER_DRIVER:138-144`: hyperfine is
//! called with `--shell=none`, `--metrics`, `--warmup`, `--runs`,
//! `--export-json`, one `--command-name` per variant and the shell-joined
//! measured commands.

use crate::config::Invariant;
use crate::HYPERFINE_VERSION;

/// A pinned Hyperfine measurement command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hyperfine {
    /// Pinned version string.
    pub version: &'static str,
    /// Warmup iterations.
    pub warmups: u32,
    /// Measured iterations.
    pub runs: u32,
    /// Metrics expression.
    pub metrics: String,
}

impl Hyperfine {
    /// Bind the pinned version to the manifest's tuning values.
    #[must_use]
    pub fn pinned(invariant: &Invariant) -> Self {
        Self {
            version: HYPERFINE_VERSION,
            warmups: invariant.warmups,
            runs: invariant.runs,
            metrics: invariant.metrics().to_owned(),
        }
    }

    /// Build the hyperfine argv (prun_lab.py:138-144).
    #[must_use]
    pub fn argv(&self, path: &str, names: &[String], commands: &[String]) -> Vec<String> {
        let mut argv = vec![
            "hyperfine".to_owned(),
            "--shell=none".to_owned(),
            "--metrics".to_owned(),
            self.metrics.clone(),
            "--warmup".to_owned(),
            self.warmups.to_string(),
            "--runs".to_owned(),
            self.runs.to_string(),
            "--export-json".to_owned(),
            path.to_owned(),
        ];
        names.iter().for_each(|name| {
            argv.push("--command-name".to_owned());
            argv.push(name.clone());
        });
        argv.extend(commands.iter().cloned());
        argv
    }
}
