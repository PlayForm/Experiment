//! A signed, preregistered bid for research capacity.
//!
//! Grounded in `schemas/research-bid.v5.schema.json:6-148` (14 required keys),
//! `examples/research-bid.v5.json:2-28`, `MARKET.md:29-38` (upper-bound
//! requested resources, explicit frozen forecasts, falsifiable mechanism,
//! novelty fingerprint, information-gain basis, planned references, confidential
//! method commitment, declared stance, unique parent case and signed budget ID)
//! and `SYSTEM.md:85` (bids are evaluated, not believed).

use crate::{case::{Prediction, Stance},credit::Resource,Digest,Permission,Warning};

/// One researcher's signed offer of work for bounded resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bid {
    /// Bid identifier (`schema:27-31`).
    pub bid_id: String,
    /// Unique parent case (`schema:32-36`, `MARKET.md:38`).
    pub case_id: String,
    /// Declared stance (`schema:42-51`).
    pub stance: Stance,
    /// Content commitment of the frozen task snapshot (`schema:52-55`).
    pub snapshot_hash: Digest,
    /// Confidential commitment to the method (`schema:56-59`, `MARKET.md:36`).
    pub proposal_hash: Digest,
    /// Native-unit upper bounds, not a spend (`schema:60-102`).
    pub requested: Resource,
    /// Deterministic novelty / duplicate fingerprint (`schema:103-107`).
    pub mechanism_key: String,
    /// Forecast frozen before the outcome (`schema:108-126`, `MARKET.md:31`).
    pub prediction: Prediction,
    /// Advisory information-gain basis (`schema:127-130`, `MARKET.md:45`).
    pub information_gain_basis: String,
    /// Planned reference coverage (`schema:131-133`).
    pub source_coverage_note: String,
    /// Correlation cohort for independence accounting (`schema:134-138`,
    /// `AGENTS.md:25-27`).
    pub model_cohort_id: String,
    /// Signer identity (`schema:37-41`).
    pub bidder_identity: String,
}

impl Bid {
    /// Validate identifiers and the frozen forecast; no signature is checked.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.bid_id.is_empty(),
            self.case_id.is_empty(),
            self.mechanism_key.is_empty(),
            self.model_cohort_id.is_empty(),
        ) {
            (false, false, false, false) => self.prediction.validate(),
            (true, _, _, _) => Err(Warning::InvalidValue {
                field: "bid_id",
                detail: "empty bid identifier".to_owned(),
            }),
            (_, true, _, _) => Err(Warning::MissingArtifact {
                digest: self.case_id.clone(),
            }),
            (_, _, true, _) => Err(Warning::InvalidValue {
                field: "mechanism_key",
                detail: "a mechanism key is required".to_owned(),
            }),
            (_, _, _, true) => Err(Warning::InvalidValue {
                field: "model_cohort_id",
                detail: "a correlation cohort is required".to_owned(),
            }),
        }
    }

    /// A self-reported confidence is never a measurement stamp
    /// (`MARKET.md:43`, `AGENTS.md:17`).
    #[must_use]
    pub const fn confidence_is_evidence() -> Permission {
        Permission::Deny
    }

    /// A conflicting opinion is allowed; a forged measurement is not
    /// (`README.md:41-43`, `MARKET.md:105-106`).
    #[must_use]
    pub const fn may_contradict_peer() -> Permission {
        Permission::Allow
    }
}
