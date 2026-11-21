//! Shadow heuristic mode: compute a plan, execute the reference.
//!
//! Grounded in `REQUIREMENTS.md:211-212`, `README.md:43-44`
//! ("static and shadow modes with reference fallback"), REQ-HEU-15
//! (`:240-241`) and `REQUIREMENTS.md:819-820` (adaptive selection starts as
//! opt-in or shadow). This type records the separation; it executes nothing.

use crate::{agent::Permission, heuristic::{Mode, Plan}};

/// Shadow-mode policy: labels are collected without changing execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shadow {
    /// Whether a candidate plan is computed for offline comparison.
    pub compute_plan: Permission,
    /// Always denied: the reference plan is what actually runs.
    pub execute_plan: Permission,
    /// Whether the computed plan is stored as a comparison label.
    pub collect_labels: Permission,
    /// Whether comparison happens offline, outside primary timing.
    pub compare_offline: Permission,
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            compute_plan: Permission::Allow,
            execute_plan: Permission::Deny,
            collect_labels: Permission::Allow,
            compare_offline: Permission::Allow,
        }
    }
}

impl Shadow {
    /// The mode that actually governs execution in shadow mode.
    #[must_use]
    pub const fn executed_mode() -> Mode {
        Mode::Off
    }

    /// Whether the shadow arm is safe for primary timing: no candidate plan
    /// may be executed and analysis cost must still be counted (REQ-HEU-14).
    #[must_use]
    pub fn is_isolated(&self) -> bool {
        self.execute_plan == Permission::Deny
            && self.compute_plan == Permission::Allow
            && self.compare_offline == Permission::Allow
    }

    /// Discard the candidate plan and keep only its label as data.
    #[must_use]
    pub fn label(&self, plan: &Plan) -> String {
        match self.collect_labels {
            Permission::Allow => format!("shadow:{}:{}", plan.policy_version, plan.rationale),
            Permission::Deny => "shadow:label-collection-disabled".to_owned(),
        }
    }
}
