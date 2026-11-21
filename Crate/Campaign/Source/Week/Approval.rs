//! Per-phase approval policy from the autonomous-week playbook.
//!
//! Grounded in `playbooks/autonomous-week.yaml:29,35,41,47,53,59,65,71,77,83`
//! and REQ-SEC-07 / REQ-AGT-18 (`REQUIREMENTS.md:923,602`). `Human` gates the
//! main-branch integration; no generated artifact can approve its own result.

/// Who may authorize a phase transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Approval {
    /// `automatic_if_valid` - the trusted coordinator validates and advances.
    Automatic,
    /// `constrained_auto` - bounded agent action inside declared permissions.
    Constrained,
    /// `automatic_if_thresholds_met` - predeclared thresholds decide.
    Threshold,
    /// `automatic_to_eligible_only` - only eligible candidates continue.
    Eligible,
    /// `automatic` - curator records evidence without changing policy.
    Recorded,
    /// `human_for_main_branch` - an explicit human review is required.
    Human,
}

impl Approval {
    /// The exact YAML token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Automatic => "automatic_if_valid",
            Self::Constrained => "constrained_auto",
            Self::Threshold => "automatic_if_thresholds_met",
            Self::Eligible => "automatic_to_eligible_only",
            Self::Recorded => "automatic",
            Self::Human => "human_for_main_branch",
        }
    }

    /// Whether a human signature is required before advancing.
    #[must_use]
    pub const fn needs_human(self) -> bool {
        matches!(self, Self::Human)
    }
}
