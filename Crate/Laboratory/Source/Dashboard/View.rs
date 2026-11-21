//! The nine plane-separated operator views.
//!
//! Grounded in `docs/12-dashboard.md:11-38` and
//! `docs/24-dual-observability-and-correspondence.md:60-71`: the GPUI/CLI
//! console has zero special privileges, reads only Plane 0 projections, and
//! uses four core GETs to follow a campaign. No view can rewrite the execution
//! graph from a model-generated explanation (`docs/12:16-18`). Every view is a
//! typed value so a private research panel can never be shown to an observer.

/// An operator-facing dashboard view.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum View {
    /// Coordinator health, queue, leases and stale-projection warning.
    Overview,
    /// Task -> proposal -> mutation -> build -> oracle -> publication graph.
    ExecutionGraph,
    /// Role, cohort, busy/quarantine state and turn counts.
    AgentMatrix,
    /// Paired trials, raw samples, error bars and drift flags.
    BenchmarkMicroscope,
    /// Certificate lifecycle by role and reason, no key material.
    TrustAudit,
    /// Claims, evidence map, contradictions and publication stage.
    ResearchPapers,
    /// Control spans beside separately labelled SUT diagnostics.
    DualObservability,
    /// Source ancestry, per-mutation patch and result hashes.
    LineageDiff,
    /// Effective signed policy, restrictions, capacity and compatibility.
    PolicyOperations,
}

impl View {
    /// The four core GETs sufficient to follow a campaign (`docs/24:62-66`).
    #[must_use]
    pub const fn core_reads() -> [&'static str; 4] {
        [
            "/v1/status",
            "/v1/events/stream",
            "/v1/experiments/{id}",
            "/v1/lineage/{id}",
        ]
    }

    /// Whether this view may render raw private conversation text.
    #[must_use]
    pub const fn raw_private_text(self) -> crate::Permission {
        crate::Permission::Deny
    }

    /// Whether a control-plane view can authorize a benchmark claim.
    #[must_use]
    pub const fn authorizes_evidence(self) -> crate::Permission {
        crate::Permission::Deny
    }
}
