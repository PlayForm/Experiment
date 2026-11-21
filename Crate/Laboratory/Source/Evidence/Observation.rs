//! A typed observation carrying its own rank and uncertainty.
//!
//! Grounded in `docs/00-system-constitution.md:49-55`,
//! `docs/17-v61-constitution.md:65-73` and
//! `docs/25-failure-analysis.md:47-54`: measurements, CPU noise, wall clock and
//! model generation are timestamped observations with replayable inputs, not
//! deterministic facts. Failure and inconclusive are first-class outcomes
//! (`docs/06-state-machines.md:90-93`, `docs/10-benchmark-mesh.md:56-62`). An
//! observation records its rank so a later reader cannot mistake an opinion for
//! a measurement.

use crate::evidence::{Rank, Verdict};

/// One ranked observation of a subject under a pinned policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observation {
    /// Hierarchical rank this observation was produced at.
    pub rank: Rank,
    /// Digest of the subject the observation concerns.
    pub subject: crate::Digest,
    /// Digest of the evidence payload, if a trustworthy digest exists.
    pub payload: Option<crate::Digest>,
    /// Recorded outcome, including failure and inconclusive.
    pub outcome: Verdict,
    /// Sequence number of the authoritative event that sealed it.
    pub event_seq: u64,
}

impl Observation {
    /// Record an observation; a missing payload yields a typed warning rather
    /// than a panic, so the caller may keep a reference-only view.
    #[must_use]
    pub fn of(
        rank: Rank,
        subject: crate::Digest,
        payload: Option<crate::Digest>,
        outcome: Verdict,
        event_seq: u64,
    ) -> Result<Self, crate::Warning> {
        match payload {
            None => Err(crate::Warning::Inconclusive {
                subject: format!("{} at rank {rank:?}", subject.as_str()),
            }),
            Some(digest) => Ok(Self {
                rank,
                subject,
                payload: Some(digest),
                outcome,
                event_seq,
            }),
        }
    }

    /// Whether this observation can itself authorize a measurement claim.
    #[must_use]
    pub const fn authoritative(&self) -> crate::Permission {
        self.rank.authorizes_measurement()
    }
}
