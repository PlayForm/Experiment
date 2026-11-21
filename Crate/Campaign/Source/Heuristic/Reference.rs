//! Reference fallback: the verified strategy used when planning cannot help.
//!
//! Grounded in LAW-008 (`REQUIREMENTS.md:73-76`), REQ-ARC-02 (`:183-185`),
//! REQ-HEU-09 (`:229-230`) and `campaign.v2.json:133` (`fallback_strategy`).
//! The reference strategy must remain available in release and specialization
//! builds; this type records that guarantee and resolves a fallback decision
//! without executing anything.

use crate::{agent::Permission, heuristic::{Analysis, Mode}};

/// The named fallback strategy for low-confidence or failed analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reference {
    /// Strategy id, for example `reference` (`campaign.v2.json:73`).
    pub id: String,
    /// Versioned identifier so a fallback is reproducible.
    pub policy_version: String,
    /// Whether the reference arm is always compiled in (REQ-ARC-02).
    pub always_available: Permission,
    /// Human-readable reason recorded under `--explain-plan` (LAW-013).
    pub rationale: String,
}

impl Reference {
    /// Whether a fallback can be used at all; never panic if it cannot.
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.always_available == Permission::Allow && !self.id.is_empty()
    }

    /// Resolve a fallback for a given mode and analysis. The decision itself
    /// is returned as a named mode, never as a raw boolean.
    #[must_use]
    pub fn resolve(&self, requested: Mode, analysis: &Analysis) -> Mode {
        match (self.is_available(), analysis.is_known(), requested.plans_actively()) {
            (true, true, true) => requested,
            _ => Mode::Off,
        }
    }
}
