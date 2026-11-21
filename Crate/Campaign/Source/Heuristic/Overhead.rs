//! Planner-overhead measurement fields.
//!
//! Grounded in REQ-HEU-14 (`REQUIREMENTS.md:237-239`) and the runtime cost
//! model at `:832-858`: expected savings must plausibly exceed analyzer and
//! planner cost, and the overhead of loading rules/models must be measured
//! cold and warm. `campaign.v2.json:129` supplies the analyzer time budget.
//! This type stores measurements as data; nothing is timed here.

/// The verdict of an overhead-versus-saving comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// Expected saving plausibly exceeds measured cost.
    Worthwhile,
    /// Measured cost eats the expected saving.
    NotWorthwhile,
    /// Missing or non-finite measurements cannot support a decision.
    Inconclusive,
}

/// Timing fields that make planner overhead part of every benchmark.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Overhead {
    /// Configured analyzer budget in microseconds (`campaign.v2.json:129`).
    pub budget_us: u64,
    /// REQ-HEU-14: time spent analyzing.
    pub analysis_elapsed_ns: u64,
    /// REQ-HEU-14: time spent planning.
    pub plan_elapsed_ns: u64,
    /// REQ-HEU-14: predicted saving from the selected plan.
    pub predicted_savings_ns: u64,
    /// REQ-HEU-14: observed total elapsed time.
    pub observed_total_elapsed_ns: u64,
    /// Cold cost of reading and loading rules or models (`REQUIREMENTS.md:858`).
    pub cold_load_ns: u64,
    /// Warm cost of the same load.
    pub warm_load_ns: u64,
}

impl Overhead {
    /// Measured analyzer plus planner cost.
    #[must_use]
    pub const fn cost_ns(&self) -> u64 {
        self.analysis_elapsed_ns
            .saturating_add(self.plan_elapsed_ns)
            .saturating_add(self.cold_load_ns)
    }

    /// Whether the configured budget was respected; a zero budget means the
    /// field was absent, which is reported as inconclusive rather than a win.
    #[must_use]
    pub fn within_budget(&self) -> Verdict {
        match self.budget_us {
            0 => Verdict::Inconclusive,
            budget => {
                let allowed = u64::from(budget).saturating_mul(1_000);
                match self.analysis_elapsed_ns <= allowed {
                    true => Verdict::Worthwhile,
                    false => Verdict::NotWorthwhile,
                }
            }
        }
    }

    /// Decide whether adaptive planning pays for itself. A zero prediction is
    /// inconclusive, never a win (REQ-BEN-07, REQ-HEU-14).
    #[must_use]
    pub fn decide(&self) -> Verdict {
        match (self.predicted_savings_ns, self.observed_total_elapsed_ns) {
            (0, _) | (_, 0) => Verdict::Inconclusive,
            (saving, _) => match saving > self.cost_ns() {
                true => Verdict::Worthwhile,
                false => Verdict::NotWorthwhile,
            },
        }
    }
}
