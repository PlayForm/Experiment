//! Mutation promotion states (`docs/06-state-machines.md:25-40`).
//!
//! Proposed -> admitted -> built -> oracle -> measured -> evaluated -> eligible
//! -> promoted, with rejected/oracle_failed/archived terminal branches. Failure
//! and inconclusive are never rewritten as a pass.

/// Mutation promotion state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MutationPhase {
    /// Staged, untrusted proposal.
    Proposed,
    /// Passed scope validation.
    Admitted,
    /// Builder attested a pinned build.
    Built,
    /// Oracle failed.
    OracleFailed,
    /// Oracle passed.
    OraclePassed,
    /// Runner attested signed measurements.
    Measured,
    /// Evaluator attested a verdict.
    Evaluated,
    /// Policy engine found it eligible.
    Eligible,
    /// Release gate promoted it.
    Promoted,
    /// Policy engine rejected it.
    Rejected,
    /// Archived with uncertainty preserved.
    Archived,
}
