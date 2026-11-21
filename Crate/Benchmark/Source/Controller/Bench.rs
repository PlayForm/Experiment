//! The `bench` subcommand.
//!
//! Grounded in `prun_lab.py:521-597`: the built matrix must still match the
//! config, fixtures are ensured, groups are formed and each group is run
//! sequentially unless disjoint leases were validated.

use super::ParallelRunners;
use crate::Fault;

/// A request to re-benchmark previously built variants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bench {
    /// Runner group concurrency.
    pub parallel: ParallelRunners,
}

impl Bench {
    /// Reject a matrix that changed since the build (prun_lab.py:524-527).
    pub fn guard(current: &[(&str, &str)], built: &[(&str, &str)]) -> Result<(), Fault> {
        (current == built).then_some(()).ok_or(Fault::Stale)
    }
}
