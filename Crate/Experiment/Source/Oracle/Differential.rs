//! Differential correctness result between candidate and frozen reference.
//!
//! Grounded in `ARCHITECTURE.md:82-86` (paired comparisons against frozen B0,
//! repeated blocks and order randomization), `REQUIREMENTS_V3.md` INV-009
//! (behavior preservation takes precedence over a speedup; failures exit
//! without promotion) and `playbooks/fanout-5.yaml:65-68,74-83` (oracle failure
//! rejects with a scoped capsule before benchmark). Categories are typed so a
//! violation cannot be reinterpreted as a win.

/// A differential outcome for one candidate/reference pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kind {
    /// Candidate matches the frozen reference on all trusted fixtures.
    Equivalent,
    /// Candidate changed observable behavior.
    Divergent,
    /// Candidate crashed or did not complete.
    Crashed,
    /// The differential could not be established.
    Undetermined,
}

/// One differential observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Differential {
    /// Candidate binary digest.
    pub candidate_sha256: String,
    /// Frozen reference binary digest.
    pub reference_sha256: String,
    /// The differential outcome.
    pub kind: Kind,
    /// Minimal violation categories, never hidden fixture content.
    pub categories: Vec<String>,
}

impl Differential {
    /// Whether the candidate may proceed to the benchmark queue.
    #[must_use]
    pub const fn may_queue(&self) -> bool {
        matches!(self.kind, Kind::Equivalent)
    }

    /// Whether this observation authorizes promotion. Only equivalence does,
    /// and even then a separate statistical verdict is required.
    #[must_use]
    pub const fn may_promote(&self) -> bool {
        matches!(self.kind, Kind::Equivalent)
    }

    /// The outcome token recorded in the ledger.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        match self.kind {
            Kind::Equivalent => "EQUIVALENT",
            Kind::Divergent => "DIVERGENT",
            Kind::Crashed => "CRASHED",
            Kind::Undetermined => "UNDETERMINED",
        }
    }
}
