//! Benchmark experiment states (`docs/06-state-machines.md:55-64`).
//!
//! Queued -> leased -> running -> sealed -> evaluated, with a distinct
//! contaminated state that returns to queued under a new attempt id. A
//! contaminated run stays in the ledger and is never silently corrected
//! (`docs/10-benchmark-mesh.md:56-62`).

/// Benchmark experiment state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BenchmarkPhase {
    /// Awaiting an exclusive lease.
    Queued,
    /// Exclusive resource leased.
    Leased,
    /// Paired trials running.
    Running,
    /// Evidence sealed and signed.
    Sealed,
    /// Drift or interference invalidated samples.
    Contaminated,
    /// Evaluator attested the result.
    Evaluated,
}
