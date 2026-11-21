//! The rayon + tokio hybrid, modelled as typed scheduling facts.
//!
//! Grounded in `Fn/Binary/Command/Parallel.rs:27-146`: classification by
//! `par_iter` (`:30-40`), target derivation by `into_par_iter` (`:42-52`),
//! an `ArrayQueue` pre-sized to the target count (`:59-62`), one Tokio worker
//! per `rayon::current_num_threads()` (`:64-72`), the global GPG mutex
//! (`:19,90-95`), and `AllDone` after every worker joins (`:141-145`).

use crate::execute::{gpg::GPG, git::Git, index::Index};
use crate::walk::entry::Entry;

/// The concurrent strategy's static contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parallel {
    pattern: String,
}

impl Parallel {
    /// Classification primitive (`Parallel.rs:33`).
    pub const CLASSIFY: &'static str = "par_iter";
    /// Queue type (`Parallel.rs:59`).
    pub const QUEUE: &'static str = "ArrayQueue";
    /// Source of the worker count (`Parallel.rs:64`).
    pub const WORKER_SOURCE: &'static str = "rayon::current_num_threads";
    /// Event kinds this strategy emits, in order of first appearance.
    pub const EMITS: [&'static str; 6] = [
        "JobStarted",
        "IndexLockTimeout",
        "Line",
        "JobProgress",
        "JobFinished",
        "AllDone",
    ];

    /// Bind the strategy to the pattern used for target matching.
    #[must_use]
    pub fn build(pattern: String) -> Self {
        Self { pattern }
    }

    /// Pre-classify commands once, as `par_iter().map(...).collect()` does
    /// (`Parallel.rs:30-40`), before any directory worker starts.
    #[must_use]
    pub fn classify(&self, commands: &[String], git: Git) -> Vec<(GPG, Index)> {
        commands
            .iter()
            .map(|command| (GPG::from_command(command, git), Index::from_command(command)))
            .collect()
    }

    /// Target derivation by `into_par_iter` (`Parallel.rs:42-52`).
    #[must_use]
    pub fn targets(&self, candidates: &[Entry]) -> Vec<String> {
        candidates
            .iter()
            .filter_map(|entry| match entry.name() == Some(self.pattern.as_str()) {
                true => entry.parent(),
                false => None,
            })
            .collect()
    }

    /// Queue capacity equals the number of targets (`Parallel.rs:59`).
    #[must_use]
    pub const fn queue_capacity(targets: usize) -> usize {
        targets
    }

    /// One Tokio worker per available Rayon thread (`Parallel.rs:64-67`).
    #[must_use]
    pub const fn workers(threads: usize) -> usize {
        threads
    }

    /// No targets: emit only `AllDone` and return (`Parallel.rs:54-57`).
    #[must_use]
    pub const fn empty_target_event() -> &'static str {
        "AllDone"
    }
}
