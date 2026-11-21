//! A versioned, signed paper with an explicit evidence-status graph.
//!
//! Grounded in `paper.v5.schema.json:6-151` (17 required keys, unique case IDs,
//! eight statuses, claim links, three digests, limitations, negative results,
//! sanitization status, optional grant, distinct publisher identity and
//! signature), `examples/paper.v5.json:2-25`,
//! `PUBLISHING.md:48-51,86-103` (machine-readable claim graph and paper format),
//! `sql/v5_migration.sql:171-201` (superseding manuscripts and paper claims)
//! and `README.md:46-49` (a research paper is never automatically peer
//! reviewed). No paper is written, sanitized or released here.

use crate::{Digest, Permission, Warning};

use super::{Link, Sanitization, Status};

/// A versioned technical artifact, not automatically a publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paper {
    /// Paper identifier (`paper schema:30-34`).
    pub paper_id: String,
    /// Motivating repository (`paper schema:35-39`).
    pub repo_id: String,
    /// Unique parent case IDs (`paper schema:40-49`).
    pub case_ids: Vec<String>,
    /// Current ladder rung (`paper schema:50-60`).
    pub status: Status,
    /// One-sentence calibrated abstract (`paper schema:66-69`).
    pub abstract_text: String,
    /// Machine-readable claim graph (`paper schema:70-103`).
    pub claim_links: Vec<Link>,
    /// Digest of the frozen baseline (`paper schema:104-107`).
    pub baseline_sha256: Digest,
    /// Digest of the method recipe (`paper schema:108-111`).
    pub methods_sha256: Digest,
    /// Digest of the manuscript bytes (`paper schema:112-115`).
    pub manuscript_sha256: Digest,
    /// Mandatory limitations (`paper schema:116-122`).
    pub limitations: Vec<String>,
    /// Negative results kept first-class (`paper schema:123-130`).
    pub negative_results: Vec<String>,
    /// Sanitization outcome (`paper schema:131-133`).
    pub sanitization: Sanitization,
    /// Optional declassification grant (`paper schema:134-136`).
    pub declassification_grant_id: Option<String>,
    /// Distinct publisher identity (`paper schema:137-141`).
    pub publisher_identity: String,
    /// Release signature label (`paper schema:146-149`).
    pub signature_b64: String,
}

impl Paper {
    /// Validate the structural constraints of the paper contract.
    pub fn validate(&self) -> Result<(), Warning> {
        let cases_unique = self
            .case_ids
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == self.case_ids.len();
        let links_valid = self.claim_links.iter().all(|link| link.validate().is_ok());
        match (
            !self.case_ids.is_empty(),
            cases_unique,
            !self.claim_links.is_empty(),
            links_valid,
            !self.limitations.is_empty(),
        ) {
            (true, true, true, true, true) => Ok(()),
            (false, _, _, _, _) => Err(Warning::MissingArtifact {
                digest: "case_ids".to_owned(),
            }),
            (_, false, _, _, _) => Err(Warning::Duplicate {
                domain: "case",
                id: self.paper_id.clone(),
            }),
            (_, _, false, _, _) => Err(Warning::MissingArtifact {
                digest: "claim_links".to_owned(),
            }),
            (_, _, _, false, _) => Err(Warning::InvalidValue {
                field: "claim_links",
                detail: "an invalid claim link was found".to_owned(),
            }),
            (_, _, _, _, false) => Err(Warning::InvalidValue {
                field: "limitations",
                detail: "at least one limitation is required".to_owned(),
            }),
        }
    }

    /// Promotion requires a named transition and signed preconditions
    /// (`README.md:46-49`, `SYSTEM.md:118-122`).
    #[must_use]
    pub const fn may_promote_without_transition() -> Permission {
        Permission::Deny
    }

    /// A paper with no independently verifiable results must visibly say
    /// hypothesis note (`PUBLISHING.md:83-84`).
    #[must_use]
    pub fn is_hypothesis_note(&self) -> bool {
        matches!(self.status, Status::Hypothesis)
    }

    /// A public release needs both a passed sanitization and a grant
    /// (`PUBLISHING.md:66-70`).
    #[must_use]
    pub fn may_release(&self) -> Permission {
        match (self.sanitization, self.declassification_grant_id.is_some()) {
            (Sanitization::Passed, true) => Permission::Allow,
            _ => Permission::Deny,
        }
    }
}
