//! The typed outcome of dispatching a subcommand.
//!
//! This is a description of what execution would do, not execution itself:
//! later crates attach the trusted, deterministic runner. Generated artifacts
//! never gain authority over measurement or policy here.

use std::path::PathBuf;

use super::{Jobs, ParallelRunners};

/// A command reduced to its model-level next step.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Dispatch {
    WriteDefaultConfig { path: PathBuf },
    PrintPlan,
    Build { jobs: Jobs },
    Bench { parallel: ParallelRunners },
    BuildThenBench { jobs: Jobs, parallel: ParallelRunners },
    Report { session: PathBuf },
}
