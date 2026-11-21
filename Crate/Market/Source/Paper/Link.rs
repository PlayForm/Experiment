//! A paper's claim-to-evidence link.
//!
//! Grounded in `paper.v5.schema.json:70-103` (each link names a claim ID,
//! unique evidence IDs and one evidence status) and `PUBLISHING.md:48-51`
//! (a machine-readable claim graph with status, claim IDs, citation IDs,
//! metrics, scope boundaries and confidence). A link is a typed association;
//! the evidence it names is never opened or trusted by this crate.

use crate::claim::Status;

/// One claim and the evidence grade attached to it in a paper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    /// Claim identifier (`paper schema:77-81`).
    pub claim_id: String,
    /// Immutable evidence IDs (`paper schema:82-90`).
    pub evidence_ids: Vec<String>,
    /// Graded evidence status (`paper schema:91-99`).
    pub evidence_status: Status,
}

impl Link {
    /// Validate the claim reference and the status of its evidence.
    pub fn validate(&self) -> Result<(), crate::Warning> {
        let unique = self
            .evidence_ids
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == self.evidence_ids.len();
        match (self.claim_id.is_empty(), unique) {
            (false, true) => Ok(()),
            (true, _) => Err(crate::Warning::MissingArtifact {
                digest: "claim_id".to_owned(),
            }),
            (_, false) => Err(crate::Warning::Duplicate {
                domain: "evidence",
                id: self.claim_id.clone(),
            }),
        }
    }

    /// Whether this link carries a measured fact (`README.md:41-43`).
    #[must_use]
    pub fn is_measured(&self) -> crate::Permission {
        self.evidence_status.is_measured_fact()
    }
}
