//! Versioned statistical verdicts, including preserved failure.
//!
//! Grounded in `playbooks/fanout-5.yaml:80-83` (classify win, loss, tie,
//! inconclusive, invalid), `REQUIREMENTS_V3.md` INV-010/EXP-009 and
//! `interfaces/feedback-capsule.v3.schema.json:36-46` (evaluation_status).
//! An LLM may summarize a verdict but never authors the authoritative numeric
//! classification.

/// The authoritative classification produced by the evaluator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Verdict {
    /// Practical gain with sufficient confidence.
    Win,
    /// Behavior preserved but performance regressed.
    Loss,
    /// No meaningful difference.
    Tie,
    /// Confidence could not be established; preserved, not rewritten.
    Inconclusive,
    /// Integrity or oracle failure; raw data retained.
    Invalid,
}

impl Verdict {
    /// All five classifications, in playbook order.
    pub const ALL: [Self; 5] = [
        Self::Win,
        Self::Loss,
        Self::Tie,
        Self::Inconclusive,
        Self::Invalid,
    ];

    /// The exact token used in the capsule and ledger.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Win => "benchmark_win",
            Self::Loss => "benchmark_loss",
            Self::Tie => "tie",
            Self::Inconclusive => "inconclusive",
            Self::Invalid => "invalid",
        }
    }

    /// Whether this verdict permits promotion to review.
    #[must_use]
    pub const fn promotable(self) -> bool {
        matches!(self, Self::Win)
    }

    /// Whether this verdict must remain discoverable as negative evidence.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Loss | Self::Inconclusive | Self::Invalid)
    }
}
