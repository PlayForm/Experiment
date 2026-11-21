//! A pre-registered research case for one atomic mutation.
//!
//! Grounded in `schemas/research-case.v5.schema.json:6-152` (16 required root
//! keys, 8 states, 6 roles, 3 privacy classes, 3 holdout policies, 7 budget
//! dimensions), `examples/research-case.v5.json:2-25`,
//! `SYSTEM.md:104-116` (branching opinion streams, single-parent mutation
//! tracks, no in-place editing of prior claims) and `sql/v5_migration.sql:7-20`.
//! A case is validated data; opening, assigning and closing are not performed.

use std::collections::BTreeSet;

use crate::{credit::Resource, Digest, Permission, Warning};

use super::{Holdout, Privacy, Question, State, Stance};

/// The frozen registration of one atomic research mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCase {
    /// Case identifier (`schema:28-32`).
    pub case_id: String,
    /// Repository identifier (`schema:33-37`).
    pub repo_id: String,
    /// Campaign identifier (`schema:38-42`).
    pub campaign_id: String,
    /// Single parent mutation (`schema:43-47`, `SYSTEM.md:109`).
    pub parent_mutation_id: String,
    /// Pinned source commit (`schema:48-52`).
    pub source_commit: String,
    /// Digest of the authorized source scope (`schema:53-56`).
    pub source_scope: Digest,
    /// Workload identifier (`schema:57-61`).
    pub workload_id: String,
    /// Lifecycle state (`schema:62-73`).
    pub state: State,
    /// The frozen question (`schema:74-76`).
    pub question: Question,
    /// Hypothesis family label (`schema:78-82`).
    pub hypothesis_family: String,
    /// At least two distinct researcher stances (`schema:83-97`).
    pub roles: Vec<Stance>,
    /// Privacy class (`schema:98-100`).
    pub privacy: Privacy,
    /// Holdout policy (`schema:101-103`).
    pub holdout: Holdout,
    /// Native-unit budget ceiling (`schema:104-146`, `MARKET.md:9`).
    pub budget_ceiling: Resource,
    /// Creation timestamp (`schema:147-150`).
    pub created_utc: String,
}

impl ResearchCase {
    /// Validate the structural rules of the v5 case contract.
    pub fn validate(&self) -> Result<(), Warning> {
        self.question.validate()?;
        let distinct = self.roles.iter().collect::<BTreeSet<_>>().len();
        match (
            self.roles.len() >= 2,
            distinct == self.roles.len(),
            self.roles.iter().all(|role| role.is_case_role()),
        ) {
            (true, true, true) => Ok(()),
            (false, _, _) => Err(Warning::InvalidValue {
                field: "roles",
                detail: "a case needs at least two researchers".to_owned(),
            }),
            (_, false, _) => Err(Warning::Duplicate {
                domain: "role",
                id: self.case_id.clone(),
            }),
            (_, _, false) => Err(Warning::InvalidValue {
                field: "roles",
                detail: "unknown or non-case role".to_owned(),
            }),
        }
    }

    /// A mutation track has one parent; branches are new case IDs
    /// (`SYSTEM.md:108-110`).
    #[must_use]
    pub const fn single_parent() -> Permission {
        Permission::Allow
    }

    /// Prior claims are never edited in place; superseding records append
    /// (`SYSTEM.md:110-111`, `PUBLISHING.md:27-28`).
    #[must_use]
    pub const fn may_edit_prior_claim() -> Permission {
        Permission::Deny
    }

    /// A trivial change may be assigned a minimal cheap researcher; escalation
    /// is policy-driven (`SYSTEM.md:113-116`). This is a typed decision input,
    /// not an executed assignment.
    #[must_use]
    pub fn needs_full_panel(&self) -> bool {
        self.roles.len() > 2
    }
}
