//! Deterministic control state machine.
//!
//! Grounded in `docs/17-v61-constitution.md:65-79` (the state transition
//! function is deterministic given the ordered authoritative log, pinned policy
//! and authenticated commands) and `docs/06-state-machines.md:78-90` (the
//! universal transaction: validate credential and scope, read revision and last
//! hash, validate the edge and guard, check idempotency, append atomically).
//! Wall-clock measurements never enter this function (`docs/00:27-29`).

use crate::{
    state::{Epoch, Event, Transition},
    Permission, Warning,
};

/// A pinned-machine state that accepts only guarded, ordered events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Machine {
    /// Current epoch; every event must match it.
    pub epoch: Epoch,
    /// Pinned policy consulted for each transition.
    pub pinned: crate::state::Policy,
    /// Last committed global sequence.
    pub seq: u64,
    /// Seen idempotency keys, replay-safe.
    pub seen: Vec<String>,
}

impl Machine {
    /// Initialize an empty machine at a pinned epoch and policy.
    #[must_use]
    pub const fn of(epoch: Epoch, pinned: crate::state::Policy) -> Self {
        Self {
            epoch,
            pinned,
            seq: 0,
            seen: Vec::new(),
        }
    }

    /// Apply one ordered event through one named transition.
    ///
    /// Refuses a stale epoch, a replayed command or an unregistered
    /// `(from,to)` edge as a typed warning; a refused event never advances
    /// `seq` and never mutates state.
    #[must_use]
    pub fn apply(
        &mut self,
        transition: Transition,
        from: &str,
        to: &str,
        claimed: &Epoch,
        event: &Event,
    ) -> Result<Permission, Warning> {
        self.epoch.check(claimed)?;
        let replayed = self.seen.iter().any(|key| key == &event.idempotency);
        match replayed {
            true => Err(Warning::Replay {
                message_id: event.idempotency.clone(),
            }),
            false => self.commit(transition, from, to, event),
        }
    }

    /// Commit a validated edge and its atomic event sequence.
    fn commit(
        &mut self,
        transition: Transition,
        from: &str,
        to: &str,
        event: &Event,
    ) -> Result<Permission, Warning> {
        match (
            transition.matches(from, to),
            self.pinned.evaluate(),
            event.seq == self.seq + 1,
        ) {
            (true, Permission::Allow, true) => {
                self.seen.push(event.idempotency.clone());
                self.seq = event.seq;
                Ok(Permission::Allow)
            }
            (false, _, _) => Err(Warning::Denied {
                subject: transition.guard(),
                reason: "unregistered (from,to) edge",
            }),
            (_, Permission::Deny, _) => Err(Warning::Denied {
                subject: transition.guard(),
                reason: "pinned policy denies",
            }),
            (_, _, false) => Err(Warning::StaleProjection {
                have: self.seq,
                want: event.seq,
            }),
        }
    }
}
