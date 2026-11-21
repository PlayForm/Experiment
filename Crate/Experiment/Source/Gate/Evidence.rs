//! Experiment-key evidence reuse: when old measurements may be cited.
//!
//! Grounded in `RESEARCH_GATE.md:51-74` (run_key and exact_build_key derivation,
//! seven cache outcomes, `EVIDENCE_REUSED` events, no cross-key verdict copy),
//! `REQUIREMENTS_V3.md` INV-006/NOV-006 and `ARCHITECTURE.md:76-80`. Evidence is
//! reusable only for an identical executable, environment, fixture, protocol
//! and reset policy; otherwise it is historical reference, never a new trial.
//!
//! Exceptions declared: `Reuse` and `Drift` are typed forms of evidence fields
//! and have no independent lifecycle outside `Evidence`.

/// The outcome of looking up a prior experiment key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Reuse {
    /// An exact result is available for the identical run key.
    ExactResultAvailable,
    /// An exact immutable artifact is available for the identical build key.
    ExactBuildAvailable,
    /// The prior artifact is not the same executable.
    PriorArtifactNotSameExecutable,
    /// A close but non-identical prior exists.
    NearDuplicate,
    /// No prior evidence; a fresh trial is required.
    New,
    /// An independent confirmation is required before use.
    RequiresConfirmation,
    /// A feature or scope conflict blocks reuse.
    Conflict,
}

/// Whether environment drift screening completed for a reuse decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Drift {
    /// Drift screening completed.
    Checked,
    /// Drift screening has not completed.
    Unchecked,
}

/// Evidence attached to one canonical experiment key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    /// The full run key, binding all environment and protocol hashes.
    pub run_key: String,
    /// The narrower build key, binding source, toolchain and flags.
    pub exact_build_key: String,
    /// The reuse outcome.
    pub reuse: Reuse,
    /// Source experiment ids cited when evidence is reused.
    pub source_experiments: Vec<String>,
    /// Whether environment drift screening passed for reuse.
    pub drift: Drift,
}

impl Evidence {
    /// The reuse token recorded in the ledger.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        match self.reuse {
            Reuse::ExactResultAvailable => "EXACT_RESULT_AVAILABLE",
            Reuse::ExactBuildAvailable => "EXACT_BUILD_AVAILABLE",
            Reuse::PriorArtifactNotSameExecutable => "PRIOR_ART_NOT_SAME_EXECUTABLE",
            Reuse::NearDuplicate => "NEAR_DUPLICATE",
            Reuse::New => "NEW",
            Reuse::RequiresConfirmation => "REQUIRES_CONFIRMATION",
            Reuse::Conflict => "CONFLICT",
        }
    }

    /// Whether an `EVIDENCE_REUSED` event must be emitted.
    #[must_use]
    pub const fn emits_reuse_event(&self) -> bool {
        matches!(
            self.reuse,
            Reuse::ExactResultAvailable
                | Reuse::ExactBuildAvailable
                | Reuse::PriorArtifactNotSameExecutable
        )
    }

    /// Whether a verdict may be copied from a cited source experiment. Only an
    /// exact result under the same run key with a completed drift check may.
    #[must_use]
    pub const fn may_copy_verdict(&self) -> bool {
        matches!(
            (self.reuse, self.drift),
            (Reuse::ExactResultAvailable, Drift::Checked)
        )
    }
}
