//! Immutable, CAS-addressed build artifact and its custody state.
//!
//! Grounded in `ARCHITECTURE.md:127-129` (build outputs are CAS-addressed and
//! read-only in runners), `STORAGE.md:7-9` (source, patches and binaries are
//! authoritative CAS objects) and `TELEMETRY.md:29-40` (trace-off, trace-lite,
//! trace-diagnostic and profile-build are separately hashed binaries; their
//! results are not interchangeable).
//!
//! Exception declared: `Custody` is the typed form of the published flag and
//! has no independent lifecycle outside `Artifact`.

/// Where an artifact sits in its immutable lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Custody {
    /// Compiled but not yet published to the CAS.
    Staged,
    /// Published and immutable; runners may only read it.
    Published,
}

/// One reproducible build output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    /// Content hash of the binary.
    pub digest: String,
    /// The exact build key that produced it.
    pub build_key: String,
    /// Instrumentation mode, part of binary identity.
    pub instrumentation_mode: String,
    /// Relative path of the binary inside the artifact.
    pub binary_relpath: String,
    /// Current custody state.
    pub custody: Custody,
}

impl Artifact {
    /// Whether this artifact may be measured in a primary release trial.
    /// Only a published `trace-off` binary qualifies
    /// (`REQUIREMENTS_V3.md` TEL-004/TEL-005).
    #[must_use]
    pub fn is_primary_releasable(&self) -> bool {
        matches!(
            (self.custody, self.instrumentation_mode.as_str()),
            (Custody::Published, "trace-off")
        )
    }

    /// Whether an already published artifact may be mutated. It never may.
    #[must_use]
    pub const fn is_immutable(&self) -> bool {
        true
    }

    /// The digest this artifact is addressed by.
    #[must_use]
    pub fn address(&self) -> &str {
        &self.digest
    }
}
