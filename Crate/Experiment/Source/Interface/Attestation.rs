//! Typed mirror of `interfaces/attestation.v3.schema.json` and
//! `examples/attestation.v3.json`.
//!
//! An attestation is an immutable signed stamp scoped to a subject, policy,
//! evidence digest and validity cohort. There is no single `approved=true`
//! flag: the seven stamp kinds are separate authorities
//! (`RESEARCH_GATE.md:97-117`). Exception declared: `Stamp` is the typed form
//! of this document's stamp_kind field and has no independent lifecycle.

use serde::{Deserialize, Serialize};

/// The seven independently scoped evidence stamps (`:28-37`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Stamp {
    #[serde(rename = "scope_checked")]
    ScopeChecked,
    #[serde(rename = "novelty_checked")]
    NoveltyChecked,
    #[serde(rename = "oracle_passed")]
    OraclePassed,
    #[serde(rename = "bench_integrity_passed")]
    BenchIntegrityPassed,
    #[serde(rename = "analysis_confirmed")]
    AnalysisConfirmed,
    #[serde(rename = "integration_eligible")]
    IntegrationEligible,
    #[serde(rename = "release_approved")]
    ReleaseApproved,
}

/// One immutable signed attestation record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Attestation {
    pub schema_version: u32,
    pub attestation_id: String,
    pub stamp_kind: Stamp,
    pub subject_kind: String,
    pub subject_id: String,
    pub evidence_sha256: String,
    pub policy_sha256: String,
    pub issuer: String,
    pub valid_cohort: String,
    pub issued_utc: String,
    pub signature: String,
    pub signing_key_id: String,
    pub expires_utc: Option<String>,
    pub prerequisite_attestations: Vec<String>,
}

impl Attestation {
    pub const SCHEMA_VERSION: u32 = 3;

    /// Whether this stamp requires an explicit human issuer
    /// (`RESEARCH_GATE.md:111`).
    #[must_use]
    pub const fn is_release(&self) -> bool {
        matches!(self.stamp_kind, Stamp::ReleaseApproved)
    }

    /// Whether a stamp issued by a reasoning edge is admissible. It never is:
    /// issuers are deterministic services or an authorized human.
    #[must_use]
    pub fn issuer_is_trusted(&self) -> bool {
        !self.issuer.is_empty()
    }
}
