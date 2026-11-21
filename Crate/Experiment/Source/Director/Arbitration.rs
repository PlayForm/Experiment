//! Scientific tradeoff arbitration outcomes available to the director.
//!
//! Grounded in `ARCHITECTURE.md:94-96` and `playbooks/fanout-5.yaml:96-104`:
//! trigger a new fanout, cancel unpromising tracks, request independent
//! integration experiments, or revise a lane as a new mutation. The director
//! selects a hypothesis direction; the deterministic coordinator still decides
//! acceptance, scheduling and release.

/// A director's arbitration of the current research state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Arbitration {
    /// Start another five-way fanout within budget.
    TriggerFanout,
    /// Retire an unpromising lane.
    CancelTrack,
    /// Request an independent integration experiment.
    RequestIntegration,
    /// Request sealed-holdout confirmation at a fixed checkpoint.
    RequestConfirmation,
    /// Revise the candidate as a new child mutation.
    ReviseMutation,
}

impl Arbitration {
    /// All arbitration outcomes, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TriggerFanout,
        Self::CancelTrack,
        Self::RequestIntegration,
        Self::RequestConfirmation,
        Self::ReviseMutation,
    ];

    /// The playbook action token (`fanout-5.yaml:99-102`).
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::TriggerFanout => "request_fanout",
            Self::CancelTrack => "retire_lane",
            Self::RequestIntegration => "request_integration",
            Self::RequestConfirmation => "request_confirmation",
            Self::ReviseMutation => "revise_as_new_mutation",
        }
    }

    /// Whether the action introduces a new child mutation rather than ending
    /// a lane.
    #[must_use]
    pub const fn is_revision(self) -> bool {
        matches!(self, Self::ReviseMutation | Self::TriggerFanout)
    }
}
