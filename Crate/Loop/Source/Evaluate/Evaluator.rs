//! The trusted evaluator wrapping the authoritative verdict.
//! Grounded in `ARCHITECTURE.md:59-60` (the evaluator may classify regression,
//! win, inconclusive or invalid but an LLM cannot author the numeric verdict)
//! and `26-acceptance-and-build-plan.md:47-50`.

use experiment::evaluate::verdict::Verdict;

/// A classification produced by the trusted evaluator only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evaluator {
    /// Authoritative classification from `experiment`.
    pub verdict: Verdict,
    /// Protocol version applied to the raw samples.
    pub protocol: String,
}

impl Evaluator {
    /// Classify using a verdict the evaluator computed, never a worker claim.
    #[must_use]
    pub fn of(verdict: Verdict, protocol: &str) -> Self {
        Self {
            verdict,
            protocol: protocol.to_owned(),
        }
    }

    /// Whether promotion to review is permitted.
    #[must_use]
    pub const fn promotable(&self) -> bool {
        self.verdict.promotable()
    }

    /// Whether the outcome must stay discoverable as negative evidence.
    #[must_use]
    pub const fn negative(&self) -> bool {
        self.verdict.is_negative()
    }

    /// The exact token used in the feedback capsule and ledger.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        self.verdict.key()
    }
}
