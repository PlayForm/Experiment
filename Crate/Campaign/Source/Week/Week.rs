//! The bounded 168-hour autonomous-week contract.
//!
//! Grounded in `playbooks/autonomous-week.yaml:1-102`,
//! `campaign.v2.json:10-45` (`duration_hours: 168`) and
//! `REQUIREMENTS.md:614-625` (a safe unattended campaign). The week is a typed
//! value: there is no wall clock, no lease and no campaign loop here.

use crate::{week::{Budget, Coordinator, Playbook}, Warning};

/// A bounded autonomous-week campaign contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Week {
    pub id: String,
    pub mission: String,
    pub base_commit: String,
    pub budget: Budget,
    pub playbook: Playbook,
    pub coordinator: Coordinator,
}

impl Week {
    /// The hard duration bound (`autonomous-week.yaml:9`, `campaign.v2.json:14`).
    pub const HOURS: u32 = 168;

    /// Construct a week from the design playbook.
    #[must_use]
    pub fn from_design() -> Self {
        let playbook = Playbook::from_design();
        Self {
            id: playbook.id.clone(),
            mission: playbook.mission.clone(),
            base_commit: playbook.base_commit.clone(),
            budget: playbook.budget.clone(),
            playbook,
            coordinator: Coordinator::default(),
        }
    }

    /// Warn if the week exceeds its own bound or loses a safety property.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        // Declared exception: a match guard is a structural pattern predicate,
        // not an `if` statement at a call site.
        match self.budget.duration_hours {
            hours if hours == Self::HOURS => {}
            hours => warnings.push(Warning::Bound {
                domain: "week",
                key: format!("duration_hours={hours}, bound={}", Self::HOURS),
            }),
        }
        self.playbook.validate(warnings);
        match self.coordinator.lease_is_recoverable() {
            true => {}
            false => warnings.push(Warning::Bound {
                domain: "week.coordinator",
                key: "lease cannot recover from an orphaned worker".to_owned(),
            }),
        }
    }

    /// Whether the contract is bounded in time, mutations and disk
    /// (LAW-010, REQ-AGT-16).
    #[must_use]
    pub fn is_bounded(&self) -> bool {
        self.budget.duration_hours <= Self::HOURS
            && self.budget.max_mutations > 0
            && self.budget.min_free_disk_gb > 0
            && self.coordinator.stops_without_erasure()
    }

    /// Phase count transcribed from the playbook.
    #[must_use]
    pub fn phase_count(&self) -> usize {
        self.playbook.phases.len()
    }
}
