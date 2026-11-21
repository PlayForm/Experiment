//! Resource and attempt budgets that bound the recursive loop.
//!
//! Grounded in `ARCHITECTURE.md:139-142` (finite lane depth, global attempt
//! cap, token budget, hourly wall cap, disk quota, watchdog), `REQUIREMENTS_V3.md`
//! AGT3-001/AGT3-012 and `RUNBOOK.md:52-58`. A lane is stopped or re-scoped when
//! repeated null results, falling expected value, falling novelty, repeated
//! semantic failures or absent confidence appear within the budget.

/// The signal that drives a stop/re-scope decision.
///
/// Exception declared: this enum is the typed form of the budget decision and
/// has no independent lifecycle outside `Quota`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopSignal {
    /// The lane stays within budget and remains promising.
    WithinBudget,
    /// Repeated null results were observed.
    NullResults,
    /// Expected-value estimate fell below cost.
    LowExpectedValue,
    /// Novelty of new candidates is falling.
    FallingNovelty,
    /// Semantic failures repeated.
    RepeatedFailure,
    /// Confidence cannot be established within the remaining budget.
    NoConfidence,
    /// The global cap or human halt fired.
    Exhausted,
}

/// The deterministic response to a stop signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Continue the lane unchanged.
    Continue,
    /// Re-scope the lane to a new hypothesis family.
    Rescope,
    /// Retire the lane.
    Halt,
}

/// Bounds on the autonomous campaign.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Quota {
    /// Maximum lineage depth on a single lane.
    pub lane_depth: u32,
    /// Maximum attempts across the whole generation.
    pub global_attempts: u32,
    /// Maximum model tokens for the generation.
    pub token_budget: u64,
    /// Maximum wall-clock minutes for the generation.
    pub wall_clock_minutes: u64,
    /// Maximum disk bytes for CAS artifacts.
    pub disk_bytes: u64,
}

impl Quota {
    /// Map a stop signal to a typed decision.
    #[must_use]
    pub const fn decide(signal: StopSignal) -> Decision {
        match signal {
            StopSignal::WithinBudget => Decision::Continue,
            StopSignal::FallingNovelty | StopSignal::NullResults => Decision::Rescope,
            StopSignal::LowExpectedValue
            | StopSignal::RepeatedFailure
            | StopSignal::NoConfidence
            | StopSignal::Exhausted => Decision::Halt,
        }
    }

    /// Remaining token budget after recorded usage.
    #[must_use]
    pub const fn remaining_tokens(self, used: u64) -> u64 {
        self.token_budget.saturating_sub(used)
    }
}
