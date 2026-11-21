//! Campaign lifecycle states (`docs/06-state-machines.md:11-23`).
//!
//! Draft -> validated -> active/paused/draining -> completed/failed/cancelled.
//! Only the guarded transitions in `Transition` may move between these values;
//! no free-form string can name a state.

/// Campaign lifecycle state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CampaignPhase {
    /// Draft, not yet validated by policy-service.
    Draft,
    /// Validated by policy-service.
    Validated,
    /// Owner-commanded active.
    Active,
    /// Owner-commanded paused.
    Paused,
    /// Draining all jobs toward completion.
    Draining,
    /// All jobs terminal.
    Completed,
    /// Critical fault stopped the campaign.
    Failed,
    /// Owner or policy cancelled it.
    Cancelled,
}
