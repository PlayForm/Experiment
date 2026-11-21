//! Rebuild a projection from an immutable checkpoint and the event ledger.
//!
//! The projection produced here is a value; the journal remains canonical and
//! is never rewritten by recovery (`docs/08-data-and-storage.md:20-24`,
//! `docs/14-runtime-and-recovery.md:36`).

use super::checkpoint::Checkpoint;
use super::divergence::Divergence;
use super::outcome::Outcome;
use crate::ledger::event::Event;

/// A recovery fold over the immutable events after a checkpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Replay {
    checkpoint: Checkpoint,
    events: Vec<Event>,
}

impl Replay {
    /// Declare a recovery from a checkpoint and the available journal events.
    #[must_use]
    pub fn of(checkpoint: Checkpoint, events: Vec<Event>) -> Self {
        Self {
            checkpoint,
            events,
        }
    }

    /// Events after the checkpoint sequence, in journal order.
    #[must_use]
    pub fn rebuild(&self) -> Vec<&Event> {
        let mut events = self
            .events
            .iter()
            .filter(|event| event.global_sequence > self.checkpoint.sequence)
            .collect::<Vec<_>>();
        events.sort_by_key(|event| event.global_sequence);
        events
    }

    /// Divergences between the checkpoint and the rebuilt projection.
    #[must_use]
    pub fn divergences(&self) -> Vec<Divergence> {
        let tail = self.rebuild().into_iter().last().and_then(|event| {
            match event.global_sequence == self.checkpoint.sequence {
                true => None,
                false => Some(Divergence::of(
                    self.checkpoint.sequence,
                    event.global_sequence,
                    Some(event.event_id.clone()),
                )),
            }
        });
        let cut = self.checkpoint.finding().map(|_| {
            Divergence::of(self.checkpoint.sequence, 0, None)
        });
        cut.into_iter().chain(tail).collect()
    }

    /// The honest recovery outcome.
    #[must_use]
    pub fn recover(&self) -> Outcome {
        self.divergences()
            .into_iter()
            .map(|divergence| divergence.classify())
            .fold(Outcome::Continue, |state, next| match (state, next) {
                (Outcome::Inconclusive, _) | (_, Outcome::Inconclusive) => Outcome::Inconclusive,
                (Outcome::Quarantine, _) | (_, Outcome::Quarantine) => Outcome::Quarantine,
                (Outcome::Degraded, _) | (_, Outcome::Degraded) => Outcome::Degraded,
                (Outcome::Continue, Outcome::Continue) => Outcome::Continue,
            })
    }
}
