//! Differential behaviour check against the frozen reference.
//! Grounded in `ARCHITECTURE.md:54-55` (independent correctness oracle) and
//! `26-acceptance-and-build-plan.md:45-46` (trusted oracle runs reference and
//! change-impact tests before any timing).

/// A differential result between the reference and a candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Differential {
    /// Digest of the reference output.
    pub reference_digest: String,
    /// Digest of the candidate output.
    pub candidate_digest: String,
    /// Bounded semantic failure categories, never full hidden fixtures.
    pub failures: Vec<String>,
}

impl Differential {
    /// Compare a candidate output with the reference output.
    #[must_use]
    pub fn of(reference_digest: &str, candidate_digest: &str, failures: Vec<String>) -> Self {
        Self {
            reference_digest: reference_digest.to_owned(),
            candidate_digest: candidate_digest.to_owned(),
            failures,
        }
    }

    /// Whether behaviour is equivalent on the frozen inputs.
    #[must_use]
    pub fn passes(&self) -> bool {
        self.reference_digest == self.candidate_digest && self.failures.is_empty()
    }

    /// Whether any failure category is safe to reveal to a worker.
    #[must_use]
    pub fn has_worker_safe_detail(&self) -> bool {
        self.failures.iter().all(|reason| !reason.contains("fixture"))
    }
}
