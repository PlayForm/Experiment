//! A trace-off raw sample with its trusted statistical classification.
//! Grounded in `00-system-constitution.md:35-36` (trace-off is the authoritative
//! baseline) and `AGENT_CONTRACT.md:58-61` (keep failures and contaminated
//! trials; quarantine on drift instead of editing data). Coupled to the
/// authoritative verdict owned by `experiment::evaluate`.

use experiment::evaluate::verdict::Verdict;

/// One immutable raw measurement and its classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sample {
    /// Digest of the raw sample artifact.
    pub raw_digest: String,
    /// Trusted statistical classification.
    pub verdict: Verdict,
    /// Telemetry mode; absent means trace-off.
    pub trace: Option<String>,
}

impl Sample {
    /// Record a trace-off sample with a trusted verdict.
    #[must_use]
    pub fn of(raw_digest: &str, verdict: Verdict) -> Self {
        Self {
            raw_digest: raw_digest.to_owned(),
            verdict,
            trace: None,
        }
    }

    /// Whether the sample was taken with tracing off.
    #[must_use]
    pub const fn is_trace_off(&self) -> bool {
        self.trace.is_none()
    }

    /// Whether the sample may serve as authoritative evidence.
    #[must_use]
    pub fn is_authoritative(&self) -> bool {
        self.is_trace_off() && !self.raw_digest.is_empty()
    }

    /// Whether the outcome must be retained as negative evidence.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.verdict.is_negative()
    }
}
