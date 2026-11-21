//! Signed result manifest and result-integrity verdict.
//!
//! Grounded in `REQUIREMENTS_V3.md` EXP-004/EXP-007/EXP-010 (every primary
//! trial stores the command spec, hashes, warmed state, order seed, raw samples
//! and environment health; every result gets a valid/contaminated/incomplete/
//! quarantined verdict; raw data retained even when invalid) and
//! `ARCHITECTURE.md:113-117` (runner pushes signed references only to the
//! coordinator).

use crate::permission::Permission;

/// The integrity verdict of one benchmark result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Integrity {
    /// All checks passed; the result may enter evaluation.
    Valid,
    /// Host or environment contamination was detected.
    Contaminated,
    /// The trial block was incomplete.
    Incomplete,
    /// The result was quarantined; raw data is retained.
    Quarantined,
}

/// A signed release of one measurement result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    /// Digest of the signed result manifest.
    pub manifest_sha256: String,
    /// Issuer identity of the signature.
    pub issuer: String,
    /// Digest of the raw samples retained alongside the verdict.
    pub raw_samples_sha256: String,
    /// The integrity verdict.
    pub integrity: Integrity,
    /// Trial ids covered by this manifest.
    pub trials: Vec<String>,
}

impl Release {
    /// Whether this result may be evaluated. Only a valid, signed manifest
    /// with retained raw samples qualifies.
    #[must_use]
    pub fn is_evaluable(&self) -> Permission {
        match (
            self.integrity,
            self.manifest_sha256.is_empty(),
            self.raw_samples_sha256.is_empty(),
            self.issuer.is_empty(),
        ) {
            (Integrity::Valid, false, false, false) => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Raw data is retained for every verdict, including invalid ones
    /// (`EXP-007`).
    #[must_use]
    pub const fn raw_retained() -> Permission {
        Permission::Allow
    }

    /// The exact integrity token recorded in the ledger.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        match self.integrity {
            Integrity::Valid => "valid",
            Integrity::Contaminated => "contaminated",
            Integrity::Incomplete => "incomplete",
            Integrity::Quarantined => "quarantined",
        }
    }
}
