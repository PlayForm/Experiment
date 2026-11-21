//! Paired workloads, order policy and sealed-holdout separation.
//!
//! Grounded in `RESEARCH_GATE.md:80-84` (predeclared comparisons, paired order
//! ABBA/BAAB, warmup/reset policy, sample counts and drift screening),
//! `REQUIREMENTS_V3.md` EXP-004/EXP-008 and `playbooks/fanout-5.yaml:74-79`.
//! Sealed holdout workloads never enter adaptive search; confirmation happens
//! at predetermined gates.
//!
//! Exception declared: `Order` and `Split` are the typed forms of two fields of
//! this one workload and have no independent lifecycle outside `Workload`.

/// The paired run order for a trial block.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Order {
    /// Baseline, candidate, candidate, baseline.
    Abba,
    /// Candidate, baseline, baseline, candidate.
    Baab,
}

/// The data split a workload belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Split {
    /// Training data may guide the next mutation.
    Training,
    /// Development data may guide the next mutation.
    Development,
    /// Holdout used only at a fixed confirmation gate.
    Holdout,
    /// Sealed validation, withheld from mutators.
    Sealed,
}

/// One paired benchmark workload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workload {
    /// Stable workload identifier.
    pub id: String,
    /// Digest of the immutable fixture manifest.
    pub fixture_sha256: String,
    /// Structured command argv, copied and never reinterpreted by an agent.
    pub command: Vec<String>,
    /// Paired order policy.
    pub order: Order,
    /// Warmup and reset policy token.
    pub reset_policy: String,
    /// Number of measured samples.
    pub samples: u32,
    /// Digest of the locked measurement protocol.
    pub protocol_sha256: String,
    /// The split this workload belongs to.
    pub split: Split,
}

impl Workload {
    /// Whether this workload may be seen by a patcher. Sealed workloads are
    /// never visible (`REQUIREMENTS_V3.md` AGT3-009).
    #[must_use]
    pub const fn visible_to_mutator(&self) -> bool {
        matches!(self.split, Split::Training | Split::Development | Split::Holdout)
    }

    /// Whether this workload may guide adaptive search. Only training and
    /// development may; holdout is confirmation-only (`EXP-008`).
    #[must_use]
    pub const fn is_adaptive(&self) -> bool {
        matches!(self.split, Split::Training | Split::Development)
    }

    /// The exact order token recorded in the trial manifest.
    #[must_use]
    pub const fn order_key(&self) -> &'static str {
        match self.order {
            Order::Abba => "ABBA",
            Order::Baab => "BAAB",
        }
    }
}
