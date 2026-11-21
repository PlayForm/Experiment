//! Versioned evidence approval stamps; there is no single `approved=true` flag.
//!
//! Grounded in `RESEARCH_GATE.md:97-117` (seven separate stamps, each an
//! immutable signed attestation, valid only in its declared cohort),
//! `interfaces/attestation.v3.schema.json:28-37` and `REQUIREMENTS_V3.md`
//! EXP-010/AGT3-012. Release approval is an explicit human action, off by
//! default. No generated artifact can issue any stamp.

use crate::permission::Permission;

/// One independently scoped evidence stamp.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Approval {
    /// Only allowed files and effects changed.
    ScopeChecked,
    /// Identity and overlap analysis performed with a policy version.
    NoveltyChecked,
    /// Correctness certificate linked to oracle fixture version.
    OraclePassed,
    /// Result hashes, lease and contamination status verified.
    BenchIntegrityPassed,
    /// Confidence interval and practical threshold on sealed/independent data.
    AnalysisConfirmed,
    /// All preceding requirements and review state satisfied.
    IntegrationEligible,
    /// Explicit authorized human approval; off by default.
    ReleaseApproved,
}

impl Approval {
    /// All seven stamps in prerequisite order (`RESEARCH_GATE.md:101-111`).
    pub const ALL: [Self; 7] = [
        Self::ScopeChecked,
        Self::NoveltyChecked,
        Self::OraclePassed,
        Self::BenchIntegrityPassed,
        Self::AnalysisConfirmed,
        Self::IntegrationEligible,
        Self::ReleaseApproved,
    ];

    /// The exact JSON token (`attestation.v3.schema.json:29-36`).
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::ScopeChecked => "scope_checked",
            Self::NoveltyChecked => "novelty_checked",
            Self::OraclePassed => "oracle_passed",
            Self::BenchIntegrityPassed => "bench_integrity_passed",
            Self::AnalysisConfirmed => "analysis_confirmed",
            Self::IntegrationEligible => "integration_eligible",
            Self::ReleaseApproved => "release_approved",
        }
    }

    /// Whether this stamp requires a human signature.
    #[must_use]
    pub const fn needs_human(self) -> Permission {
        match self {
            Self::ReleaseApproved => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Whether a stamp set contains this stamp.
    #[must_use]
    pub fn held_by(self, held: &[Self]) -> Permission {
        match held.contains(&self) {
            true => Permission::Allow,
            false => Permission::Deny,
        }
    }

    /// Whether all prerequisites up to and including this stamp are held.
    #[must_use]
    pub fn chain_satisfied(self, held: &[Self]) -> Permission {
        let missing = Self::ALL
            .iter()
            .take_while(|stamp| **stamp <= self)
            .any(|stamp| !held.contains(stamp));
        match missing {
            true => Permission::Deny,
            false => Permission::Allow,
        }
    }
}
