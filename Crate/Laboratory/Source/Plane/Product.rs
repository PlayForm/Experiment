//! Plane 2: product/SUT evidence and its instrumentation modes.
//!
//! Grounded in `docs/07-dual-telemetry.md:104-118`,
//! `docs/00-system-constitution.md:35-36` and `README.md:14-16`: the primary
//! timed binary is trace-off; rich spans belong to a separately tagged
//! diagnostic build and are never eligible for primary comparison; an
//! instrumentation-overhead experiment measures `lite` against `off`; PGO/perf
//! profiles carry full lineage. A dropped diagnostic trace cannot change a
//! trusted measurement. The variant is a typed value so the mode cannot be
//! silently changed at a call site.

/// Instrumentation posture of a product build.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Product {
    /// Target telemetry compiled out; authoritative baseline.
    TraceOff,
    /// Bounded counters in a separate explicit overhead experiment.
    Lite,
    /// Rich spans under an independently tagged, non-comparable build.
    Diagnostic,
    /// Training-only profile collection with full artifact lineage.
    Profile,
}

impl Product {
    /// Whether measurements from this build may authorize a performance claim.
    #[must_use]
    pub const fn authoritative(self) -> crate::Permission {
        match self {
            Self::TraceOff => crate::Permission::Allow,
            Self::Lite | Self::Diagnostic | Self::Profile => crate::Permission::Deny,
        }
    }

    /// The dashboard label that must accompany a rendered result
    /// (`docs/07:128-133`, `docs/12-dashboard.md:23-24`).
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::TraceOff => "authoritative trace-off measurement",
            Self::Lite => "instrumentation-overhead experiment, not authoritative",
            Self::Diagnostic => "diagnostic-not authoritative timing",
            Self::Profile => "training profile, not a performance comparison",
        }
    }
}
