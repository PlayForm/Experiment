//! The trusted outcome of one studied assignment.
//!
//! A lesson may be `inconclusive`; that outcome is retained rather than
//! rewritten into a pass (`docs/27:15-17`; `examples/v61-study-outcome`,
//! `verdict: inconclusive`). A model's own claim is never a verdict.

/// A determination made only by the trusted evaluator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// The candidate met the predeclared acceptance criteria.
    Pass,
    /// The candidate failed them; the regression is preserved.
    Fail,
    /// Evidence was insufficient; the uncertainty is preserved.
    Inconclusive,
}

impl Verdict {
    /// The wire spelling from the study-outcome example.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Inconclusive => "inconclusive",
        }
    }

    /// Whether the verdict may be used to promote a candidate.
    #[must_use]
    pub fn promotes(&self) -> crate::permission::Permission {
        match self {
            Self::Pass => crate::permission::Permission::Allow,
            Self::Fail | Self::Inconclusive => crate::permission::Permission::Deny,
        }
    }
}
