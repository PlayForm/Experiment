//! The `campaign` object: identity, mode, duration and governance.
//!
//! Grounded in `campaign.v2.json:10-45`, the schema block at
//! `schemas/campaign.v2.schema.json:60-229`, `autonomous-week.yaml:2-22` and
//! `REQUIREMENTS.md:614-625` (a safe unattended campaign). Budgets and safety
//! reuse the `Week::Budget` and `Safety` models so the two documents agree.
//!
//! Exception declared: `Mode` is declared beside `Charter` because it is the
//! typed form of this one field and has no independent lifecycle.

use serde::{Deserialize, Serialize};

use crate::{
    agent::Permission,
    campaign::{Objective, Safety},
    week::Budget,
    Warning,
};

/// How much authority the campaign is given (`campaign.v2.json:13`,
/// `schemas/campaign.v2.schema.json:70-77`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    /// A human drives every step.
    #[serde(rename = "manual")]
    Manual,
    /// Agents propose; a human reviews promotion (`human_reviewed_autonomous`).
    #[serde(rename = "human_reviewed_autonomous")]
    Reviewed,
    /// Fully trusted local autonomy (`fully_trusted_local`) - design only.
    #[serde(rename = "fully_trusted_local")]
    Trusted,
}

impl Mode {
    /// The JSON enum token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Reviewed => "human_reviewed_autonomous",
            Self::Trusted => "fully_trusted_local",
        }
    }

    /// Whether a human must review promotion (`REQ-SEC-07`).
    #[must_use]
    pub const fn needs_review(self) -> Permission {
        match self {
            Self::Trusted => Permission::Deny,
            Self::Manual | Self::Reviewed => Permission::Allow,
        }
    }
}

/// The campaign charter nested in the v2 root document.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Charter {
    #[serde(rename = "id")]
    pub id: String,
    pub description: String,
    pub mode: Mode,
    #[serde(rename = "duration_hours")]
    pub duration_hours: u32,
    #[serde(rename = "workload_split_seed", default)]
    pub workload_split_seed: u64,
    pub objectives: Objective,
    pub budgets: Budget,
    pub safety: Safety,
}

impl Charter {
    /// Recognised keys (`schemas/campaign.v2.schema.json:220-227`).
    pub const KEYS: [&'static str; 8] = [
        "id",
        "description",
        "mode",
        "duration_hours",
        "workload_split_seed",
        "objectives",
        "budgets",
        "safety",
    ];

    /// Warn on a duration that does not match the bounded 168-hour playbook
    /// (`autonomous-week.yaml:9`, `campaign.v2.json:14`).
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        // Declared exception: a match guard is a structural pattern predicate,
        // not an `if` statement at a call site.
        match self.duration_hours {
            0 => warnings.push(Warning::Range {
                domain: "campaign",
                key: "duration_hours".to_owned(),
            }),
            hours if hours != crate::WEEK_HOURS => warnings.push(Warning::Bound {
                domain: "campaign",
                key: format!("duration_hours={hours}, playbook={}", crate::WEEK_HOURS),
            }),
            _ => {}
        }
        match self.need_review() {
            Permission::Deny => warnings.push(Warning::Designed {
                field: "campaign.mode=fully_trusted_local",
                contract: "trusted autonomy is not implemented",
            }),
            Permission::Allow => {}
        }
    }

    /// Human review requirement of the configured mode.
    #[must_use]
    pub fn need_review(&self) -> Permission {
        self.mode.needs_review().and(self.safety.release_is_supervised())
    }
}
