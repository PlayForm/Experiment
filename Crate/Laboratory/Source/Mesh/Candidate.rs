//! One candidate measurement point on the Pareto frontier.
//!
//! Grounded in `docs/00-system-constitution.md:14-15`,
//! `docs/10-benchmark-mesh.md:30-39` (latency, CPU, RSS and throughput are
//! separate objectives) and `:44-49` (a sample binds binary, workload, CPU set,
//! cache state and ordering). A point is never a single global score: it stays
//! tied to its hardware class and preserves regressions.

/// A multi-objective measurement for one implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    /// Digest of the built binary under test.
    pub binary: crate::Digest,
    /// Hardware class the measurements were taken on.
    pub hardware_class: String,
    /// Elapsed wall time in nanoseconds.
    pub elapsed_ns: u64,
    /// CPU work in nanoseconds.
    pub cpu_ns: u64,
    /// Peak resident set size in bytes.
    pub peak_rss: u64,
    /// Verdict of the trusted oracle and evaluator.
    pub verdict: crate::evidence::Verdict,
}

impl Candidate {
    /// Record a candidate; correctness is a precondition of Pareto membership.
    #[must_use]
    pub const fn of(
        binary: crate::Digest,
        hardware_class: String,
        elapsed_ns: u64,
        cpu_ns: u64,
        peak_rss: u64,
        verdict: crate::evidence::Verdict,
    ) -> Self {
        Self {
            binary,
            hardware_class,
            elapsed_ns,
            cpu_ns,
            peak_rss,
            verdict,
        }
    }

    /// Whether this candidate dominates `other` on every objective.
    #[must_use]
    pub fn dominates(&self, other: &Self) -> bool {
        self.hardware_class == other.hardware_class
            && self.elapsed_ns <= other.elapsed_ns
            && self.cpu_ns <= other.cpu_ns
            && self.peak_rss <= other.peak_rss
            && (self.elapsed_ns < other.elapsed_ns
                || self.cpu_ns < other.cpu_ns
                || self.peak_rss < other.peak_rss)
    }
}
