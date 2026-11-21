//! Constitutional invariants - the permanent "what is optimized" values.
//!
//! Grounded in `docs/00-system-constitution.md:3-15` and
//! `docs/17-v61-constitution.md:6-35`: runtime correctness first, then wall
//! time/CPU/memory/throughput per declared workload and hardware class;
//! behavioral equivalence is evidence but never proof of absence of defects;
//! build time and model tokens are operational costs, never objective terms;
//! the full Pareto frontier and targeted implementations are preserved.
//! Each invariant is a typed value so the rule applies to every file in this
//! crate, not only the paths the documents happened to inspect.

use std::fmt;

/// A constitutional invariant the laboratory never violates.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Invariant {
    /// Optimize SUT time, CPU, memory, throughput and predictability per class.
    ObjectiveIsSut,
    /// Behavioral equivalence with the approved reference is required.
    BehavioralEquivalence,
    /// Tests are evidence; they cannot prove absence of all defects.
    TestsAreEvidence,
    /// Build time and model tokens are recorded but never scored.
    SearchCostNotObjective,
    /// Preserve the full Pareto frontier and specialized implementations.
    ParetoPreserved,
    /// The trusted kernel applies pinned policy, not natural-language rules.
    KernelIsDeterministic,
    /// No promotion on unsigned measurement or an LLM-only verdict.
    SignedMeasurementRequired,
    /// Wall-clock and hardware readings are noisy observations, never control.
    MeasurementIsObservation,
    /// Trace-off is authoritative; tracing is only a separate diagnostic.
    TraceOffBaseline,
    /// No claim about an unpublished agent's internal thought process.
    NoThoughtClaims,
    /// Old attestations stay valid for their old pinned policy.
    RevisionsAreAdditive,
    /// Canonical artifacts are immutable; corrections supersede.
    ImmutableArtifacts,
}

impl Invariant {
    /// The grounding section this invariant comes from.
    #[must_use]
    pub const fn source(self) -> &'static str {
        match self {
            Self::ObjectiveIsSut | Self::BehavioralEquivalence | Self::TestsAreEvidence => {
                "docs/00:3-9"
            }
            Self::SearchCostNotObjective => "docs/00:10-13, docs/17:24",
            Self::ParetoPreserved => "docs/00:14-15, docs/10:44-49",
            Self::KernelIsDeterministic => "docs/00:26-28, docs/17:65-73",
            Self::SignedMeasurementRequired => "docs/00:31-32, docs/17:90",
            Self::MeasurementIsObservation => "docs/17:68-73, docs/25:47-49",
            Self::TraceOffBaseline => "docs/00:35-36, docs/07:104-113",
            Self::NoThoughtClaims => "docs/00:37-39, docs/22:29-39",
            Self::RevisionsAreAdditive => "docs/00:57-62",
            Self::ImmutableArtifacts => "docs/00:61-62, docs/08:66-68",
        }
    }

    /// Render one human-readable statement of the invariant.
    #[must_use]
    pub const fn line(self) -> &'static str {
        match self {
            Self::ObjectiveIsSut => "optimize SUT metrics per workload and hardware class",
            Self::BehavioralEquivalence => "require conformance to the approved reference",
            Self::TestsAreEvidence => "tests are evidence, never proof of all defects",
            Self::SearchCostNotObjective => "build time and tokens are not objective terms",
            Self::ParetoPreserved => "preserve the full frontier and specialty candidates",
            Self::KernelIsDeterministic => "trusted software applies only pinned policy",
            Self::SignedMeasurementRequired => "no promotion without signed measurement",
            Self::MeasurementIsObservation => "measurements are observations, not control",
            Self::TraceOffBaseline => "trace-off is the authoritative timing baseline",
            Self::NoThoughtClaims => "no claim about unpublished private reasoning",
            Self::RevisionsAreAdditive => "old policy pins remain valid across revisions",
            Self::ImmutableArtifacts => "canonical artifacts are immutable; supersede",
        }
    }
}

impl fmt::Display for Invariant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.line())
    }
}
