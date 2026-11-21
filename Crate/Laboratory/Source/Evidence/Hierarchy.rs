//! Deterministic resolution of competing evidence.
//!
//! Grounded in `docs/00-system-constitution.md:49-55` and
//! `docs/17-v61-constitution.md:4,25-35`: the hierarchy is total and ordered,
//! selection is fixed software, and an independent opinion never becomes a
//! statistical replication merely because several workers agree
//! (`docs/15` BND-024, `docs/25` F09). Resolution reduces an iterator of
//! observations by rank then by authoritative event sequence - a named chain,
//! not a call-site loop - and refuses to manufacture a pass from lower ranks.

use crate::evidence::{Observation, Rank};

/// A typed resolver over the evidence hierarchy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Hierarchy;

impl Hierarchy {
    /// Resolve the winning observation of a subject.
    ///
    /// Ties at the same rank are broken by the higher authoritative event
    /// sequence, so the function is deterministic under reordered input. An
    /// empty iterator yields `None` and the caller preserves inconclusive.
    #[must_use]
    pub fn resolve<'a>(
        observations: impl Iterator<Item = &'a Observation>,
    ) -> Option<&'a Observation> {
        observations.max_by(|left, right| {
            left.rank
                .cmp(&right.rank)
                .then_with(|| left.event_seq.cmp(&right.event_seq))
        })
    }

    /// Whether any observation at or above `rank` is authoritative.
    #[must_use]
    pub fn any_authoritative<'a>(
        observations: impl Iterator<Item = &'a Observation>,
        rank: Rank,
    ) -> crate::Permission {
        let found = observations.any(|item| {
            item.rank >= rank && item.rank.authorizes_measurement() == crate::Permission::Allow
        });
        crate::Permission::from(found)
    }
}
