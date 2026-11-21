//! The `bench` dispatch: the stale-matrix guard and the runner concurrency
//! policy are delegated to `Crate/Benchmark`.
//!
//! Grounded in `Research/prun_lab/README.md` (53, 64-82, 119-123): re-benchmark
//! built variants only while the matrix still matches; comparisons inside a
//! group stay sequential and release/diagnostic groups overlap only on
//! disjoint CPU sets. `Bench::guard` and `ParallelRunners::validate` in the
//! Benchmark crate own those rules; this module only reports their outcome.

use crate::warning::failure::Failure;

/// The typed outcome of a re-benchmark request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Bench {
    /// The guard passed; carry the Benchmark model's next step.
    Ready {
        dispatch: benchmark::controller::Dispatch,
    },
    /// The matrix changed since build; carry the Benchmark guard's line.
    Stale { line: String },
}

impl Bench {
    /// Delegate the guard to the Benchmark `bench` controller.
    #[must_use]
    pub fn of(
        current: &[(&str, &str)],
        built: &[(&str, &str)],
        parallel: benchmark::controller::ParallelRunners,
    ) -> Self {
        match benchmark::controller::Bench::guard(current, built) {
            Ok(()) => Self::Ready {
                dispatch: benchmark::controller::Dispatch::Bench { parallel },
            },
            Err(fault) => Self::Stale {
                line: fault.line(),
            },
        }
    }

    /// Validate group leases through the Benchmark runner policy. A fault is
    /// returned as a typed `Failure`, never a panic.
    pub fn leases(
        parallel: benchmark::controller::ParallelRunners,
        groups: &[benchmark::runner::Group],
        invariant: &benchmark::config::Invariant,
    ) -> Result<Vec<benchmark::runner::Lease>, Failure> {
        parallel.validate(groups, invariant).map_err(Failure::from)
    }
}
