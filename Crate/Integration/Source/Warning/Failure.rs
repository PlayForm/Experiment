//! Typed degradations for the composed command surface.
//!
//! Grounded in the defensive rule of this card and in
//! `Research/prun_laboratory_v6_1/docs/26-acceptance-and-build-plan.md`
//! (82-89): the package is contracts and validation, not a claim of a
//! compiled, proven application. Every failure path here is a value the
//! caller may log; none panics and none fabricates a measurement or release.

/// A typed degradation returned instead of an abort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Failure {
    /// The config path could not be read (`prun_lab.py:190`).
    MissingConfig { path: String },
    /// The config was present but did not decode into the typed model.
    MalformedConfig { path: String },
    /// The system-under-test binary or its required argument was absent.
    AbsentBinary { name: String },
    /// A crate selector named no member of the composed chain.
    UnknownCrate { name: String },
    /// The subcommand token was outside init/plan/build/bench/all/loop.
    UnknownCommand { token: String },
    /// A typed fault propagated from the Benchmark model.
    Benchmark { line: String },
}

impl Failure {
    /// Render one operator-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::MissingConfig { path } => format!("cannot read config {path}; using defaults"),
            Self::MalformedConfig { path } => format!("malformed config {path}; using defaults"),
            Self::AbsentBinary { name } => format!("binary {name} unavailable; bench degraded"),
            Self::UnknownCrate { name } => format!("unknown crate {name}; ignored"),
            Self::UnknownCommand { token } => format!("unknown command {token}; nothing dispatched"),
            Self::Benchmark { line } => line.clone(),
        }
    }

    /// Render the degradation as a warning, never as a worker status or a
    /// measurement claim.
    #[must_use]
    pub fn warning(&self) -> String {
        format!("warning: {}", self.line())
    }
}

impl From<benchmark::Fault> for Failure {
    fn from(fault: benchmark::Fault) -> Self {
        Self::Benchmark {
            line: fault.line(),
        }
    }
}
