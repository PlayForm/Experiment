//! Named budget caps and an explicit overrun verdict.
//!
//! Grounded in `playbooks/autonomous-week.yaml:8-22`,
//! `campaign.v2.json:24-35` and REQ-AGT-16 (`REQUIREMENTS.md:596-599`). A cap
//! is a typed limit; observing a value never pauses a campaign here, it only
//! returns a verdict for a trusted coordinator to act on.
//!
//! Exception declared: `Verdict` is declared beside `Cap` because it is the
//! result of this one type and has no independent lifecycle.

use crate::week::Budget;

/// Whether an observed value is inside a cap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    Inside,
    /// The cap was exceeded; the campaign must checkpoint and stop.
    Exceeded,
}

/// One named resource cap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cap {
    pub name: &'static str,
    pub limit: u32,
    pub unit: &'static str,
}

impl Cap {
    /// Build the caps from a budget policy. Every value comes from the policy,
    /// not from a hidden constant in this module.
    #[must_use]
    pub fn from_budget(budget: &Budget) -> [Self; 10] {
        [
            Self { name: "duration", limit: budget.duration_hours, unit: "hours" },
            Self { name: "mutations", limit: budget.max_mutations, unit: "count" },
            Self { name: "builds", limit: budget.max_builds, unit: "count" },
            Self { name: "trial_blocks", limit: budget.max_trial_blocks, unit: "count" },
            Self { name: "agents", limit: budget.max_concurrent_agents, unit: "concurrent" },
            Self { name: "builds_concurrent", limit: budget.max_concurrent_builds, unit: "concurrent" },
            Self { name: "timing_jobs", limit: budget.max_timing_jobs_per_host, unit: "per_host" },
            Self { name: "build_attempts", limit: budget.max_build_attempts, unit: "per_mutation" },
            Self { name: "disk", limit: budget.max_disk_gb, unit: "GB" },
            Self { name: "free_disk", limit: budget.min_free_disk_gb, unit: "GB" },
        ]
    }

    /// Render one cap for a status line.
    #[must_use]
    pub fn line(&self) -> String {
        format!("{} <= {} {}", self.name, self.limit, self.unit)
    }

    /// Compare an observed value against this cap.
    #[must_use]
    pub const fn check(&self, observed: u32) -> Verdict {
        match observed > self.limit {
            true => Verdict::Exceeded,
            false => Verdict::Inside,
        }
    }
}
