//! Design/implementation status overlay for v2 fields.
//!
//! The bundle is a requirements and system-design extension, not a replacement
//! executable: the heuristic planner, agent scheduler, autonomous week, PGO
//! and hardware-aware release are **not yet implemented** (`README.md:5-9`,
//! `:51-58`, `REQUIREMENTS.md:1145-1149`). Every modeled field can therefore be
//! annotated with whether it is only a future-facing contract, an implemented
//! model, or a verified claim. This preserves the distinction the card asks
//! for without producing any YAML, SQL or JSON.
//!
//! REQ-CLI-06 (`REQUIREMENTS.md:829-830`) requires status output to distinguish
//! `designed`, `implemented` and `verified` capabilities; these variants are
//! that taxonomy as data.

use std::fmt;

/// The status of a modeled capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Overlay {
    /// A future-facing interface contract with no implementation.
    Designed,
    /// Represented as a typed model in this crate, not executed.
    Implemented,
    /// Backed by measurement evidence; no field in this design bundle claims it.
    Verified,
}

impl Overlay {
    /// The named fields that the bundle explicitly marks as design-only.
    pub const DESIGN_ONLY: [(&'static str, &'static str); 5] = [
        (
            "agents.playbook",
            "autonomous-week.yaml is not executable by prun_lab.py v1",
        ),
        (
            "heuristics.feature_family",
            "proposed Cargo features are absent from upstream",
        ),
        (
            "campaign.mode=fully_trusted_local",
            "no trusted autonomous loop is implemented",
        ),
        ("storage.append_only_evidence", "SQLite/WAL schema is illustrative"),
        (
            "hardware.hardware_profile_artifact",
            "hardware-aware release is a future phase",
        ),
    ];

    /// Status token used by machine-readable status output.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Designed => "designed",
            Self::Implemented => "implemented",
            Self::Verified => "verified",
        }
    }

    /// Whether this status may appear in a release claim
    /// (`REQUIREMENTS.md:1048-1053`: claims need exact scope).
    #[must_use]
    pub const fn claimable(self) -> bool {
        matches!(self, Self::Verified)
    }
}

impl fmt::Display for Overlay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.token())
    }
}
