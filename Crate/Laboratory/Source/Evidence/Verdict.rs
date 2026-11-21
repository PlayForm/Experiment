//! Verdicts including failure and inconclusive.
//!
//! Grounded in `docs/00-system-constitution.md:53-55`
//! ("Preserve failure and inconclusive states"), `docs/06-state-machines.md:93`
//! (irreconcilable outcomes are inconclusive/quarantined, never automatic
//! success), `docs/10-benchmark-mesh.md:56-62` and
//! `docs/25-failure-analysis.md:36-49` (no finite suite proves absence of all
//! bugs; a deterministic function does not give deterministic timings). A
//! missing metric is rendered as missing, never interpolated
//! (`docs/07-dual-telemetry.md:128-133`).

/// The outcome of a trusted evaluation, oracle or gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Verdict {
    /// Declared criteria were met under a pinned policy.
    Pass,
    /// Declared criteria were not met; regression or correctness failure.
    Fail,
    /// Samples were contaminated and excluded, not corrected.
    Contaminated,
    /// Evidence was absent or insufficient; uncertainty is preserved.
    Inconclusive,
}

impl Verdict {
    /// Whether this verdict may be shown as a successful promotion.
    #[must_use]
    pub const fn is_pass(self) -> crate::Permission {
        match self {
            Self::Pass => crate::Permission::Allow,
            Self::Fail | Self::Contaminated | Self::Inconclusive => crate::Permission::Deny,
        }
    }

    /// Whether this verdict closes a gate rather than deferring it.
    #[must_use]
    pub const fn is_terminal(self) -> crate::Permission {
        match self {
            Self::Pass | Self::Fail => crate::Permission::Allow,
            Self::Contaminated | Self::Inconclusive => crate::Permission::Deny,
        }
    }

    /// Stable display label; missing evidence is never silently interpolated.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Contaminated => "contaminated / excluded",
            Self::Inconclusive => "inconclusive / missing evidence",
        }
    }
}
