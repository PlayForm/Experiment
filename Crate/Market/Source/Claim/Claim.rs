//! A signed claim registered against a research case.
//!
//! Grounded in `schemas/claim.v5.schema.json:6-107` (15 required keys: version,
//! claim/case IDs, researcher identity, stance, claim type, body, confidence,
//! supports, contradicts, optional evidence and source, context digest,
//! timestamp and signature), `examples/claim.v5.json:2-16`,
//! `sql/v5_migration.sql:60-80` (immutable claims, superseding chain) and
//! `SYSTEM.md:110-111` (no in-place editing; append a signed superseding record).

use crate::{case::Stance, Digest, Permission, Warning};

use super::{Kind, Signature, Status};

/// One identity-bound, signed research claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claim {
    /// Claim identifier (`schema:27-31`).
    pub claim_id: String,
    /// Parent case (`schema:32-36`).
    pub case_id: String,
    /// Signing researcher identity (`schema:37-41`).
    pub researcher_identity: String,
    /// Declared stance (`schema:42-51`).
    pub stance: Stance,
    /// Claim kind (`schema:52-61`).
    pub kind: Kind,
    /// Claim body, at least 12 characters (`schema:62-65`).
    pub body: String,
    /// Calibrated confidence in thousandths, 0..=1000 (`schema:66-70`).
    pub confidence_milli: u32,
    /// Claim IDs this claim supports (`schema:71-77`).
    pub supports: Vec<String>,
    /// Claim IDs this claim contradicts (`schema:79-85`).
    pub contradicts: Vec<String>,
    /// Optional independent evidence digest (`schema:87-90`).
    pub evidence_sha256: Option<Digest>,
    /// Optional checked source reference (`schema:91-93`).
    pub source_ref_id: Option<String>,
    /// Content commitment of the context capsule (`schema:94-97`).
    pub context_snapshot: Digest,
    /// Registration timestamp (`schema:98-101`).
    pub registered_utc: String,
    /// Signature over the deterministic payload (`schema:102-105`).
    pub signature: Signature,
    /// Graded evidence status of this claim (`paper schema:91-99`).
    pub status: Status,
}

impl Claim {
    /// Minimum body length from the schema (`schema:64`).
    pub const MIN_BODY: usize = 12;
    /// Full confidence in thousandths (`schema:69`).
    pub const CERTAIN: u32 = 1_000;

    /// Validate the claim contract without verifying its signature.
    pub fn validate(&self) -> Result<(), Warning> {
        self.signature.validate()?;
        match (
            self.body.len() >= Self::MIN_BODY,
            self.confidence_milli <= Self::CERTAIN,
            self.claim_id.is_empty(),
            self.case_id.is_empty(),
        ) {
            (true, true, false, false) => Ok(()),
            (false, _, _, _) => Err(Warning::InvalidValue {
                field: "body",
                detail: format!("shorter than {}", Self::MIN_BODY),
            }),
            (_, false, _, _) => Err(Warning::InvalidValue {
                field: "confidence",
                detail: format!("{} milli > {}", self.confidence_milli, Self::CERTAIN),
            }),
            (_, _, true, _) => Err(Warning::MissingArtifact {
                digest: "claim_id".to_owned(),
            }),
            (_, _, _, true) => Err(Warning::MissingArtifact {
                digest: "case_id".to_owned(),
            }),
        }
    }

    /// A claim may be independently replicated or contradicted, but its prior
    /// bytes are never edited in place (`SYSTEM.md:108-111`).
    #[must_use]
    pub const fn supersede_in_place() -> Permission {
        Permission::Deny
    }

    /// Only measured evidence can raise this claim's status
    /// (`README.md:41-43`, `paper schema:91-99`).
    #[must_use]
    pub fn is_measured_fact(&self) -> Permission {
        self.status.is_measured_fact()
    }
}
