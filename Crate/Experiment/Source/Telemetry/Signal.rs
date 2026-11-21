//! Instrumentation modes and low-cardinality control signals.
//!
//! Grounded in `TELEMETRY.md:27-40` (trace-off, trace-lite, trace-diagnostic,
//! profile-build are separately hashed binaries), `REQUIREMENTS_V3.md`
//! TEL-002/TEL-004/TEL-005 and `TELEMETRY.md:67-89` (metric labels must stay
//! low-cardinality; campaign and patch ids belong in trace attributes).
//! Exceptions declared: `Mode` and `Cardinality` are typed forms of this one
//! signal and have no independent lifecycle outside `Signal`.

use crate::telemetry::plane::Plane;

/// The compilation/instrumentation mode of a binary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Mode {
    /// Product probes eliminated; the only mode allowed for primary timing.
    TraceOff,
    /// Small preallocated counters, separate calibrated cohort.
    TraceLite,
    /// Full tracing spans on a diagnostic host, never primary proof.
    TraceDiagnostic,
    /// PGO or LLVM profiling build, training set only.
    ProfileBuild,
}

impl Mode {
    /// The exact build/run identity token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::TraceOff => "trace-off",
            Self::TraceLite => "trace-lite",
            Self::TraceDiagnostic => "trace-diagnostic",
            Self::ProfileBuild => "profile-build",
        }
    }

    /// Whether this mode may be compared for primary release speed.
    #[must_use]
    pub const fn is_primary(self) -> bool {
        matches!(self, Self::TraceOff)
    }
}

/// Whether a label set is safe for a metric.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cardinality {
    /// Bounded set: role, host class, status, policy version.
    Bounded,
    /// Unbounded id: campaign, run, patch, trial; trace attribute only.
    Unbounded,
}

/// One emitted telemetry signal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signal {
    /// Stable metric or span name.
    pub name: String,
    /// The plane this signal belongs to.
    pub plane: Plane,
    /// Cardinality class of this signal's labels.
    pub cardinality: Cardinality,
    /// Instrumentation mode of the measured binary.
    pub mode: Mode,
}

impl Signal {
    /// Whether this signal may carry an experiment id as a metric label.
    #[must_use]
    pub const fn may_label_id(&self) -> bool {
        matches!(self.cardinality, Cardinality::Bounded)
    }

    /// Whether the signal's binary mode is a primary release candidate.
    #[must_use]
    pub const fn is_primary(&self) -> bool {
        self.mode.is_primary()
    }
}
