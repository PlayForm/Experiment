//! Trusted, frozen-commit correctness oracle; independent of patcher edits.
//!
//! Grounded in `ARCHITECTURE.md:54-55,127-129` (tests and fixtures come from
//! frozen trusted commits; an agent-proposed oracle adjustment is a separate
//! reviewable mutation), `REQUIREMENTS_V3.md` EXP-003/EXP-006/EXP-010 and
//! `playbooks/fanout-5.yaml:65-68`. A candidate cannot redefine pass/fail
//! criteria inside its own mutation.
//!
//! Exception declared: `Outcome` is the typed form of this one oracle result
//! and has no independent lifecycle outside `Oracle`.

use crate::permission::Permission;

/// The outcome of a trusted correctness check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// The artifact satisfies the frozen semantic contract.
    Pass,
    /// The artifact violated the contract; only a minimal category is exposed.
    Fail,
    /// The oracle could not establish an outcome.
    Inconclusive,
}

/// A purpose-bound correctness oracle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Oracle {
    /// Stable oracle identity, recorded on the oracle_passed attestation.
    pub id: String,
    /// The frozen trusted commit the fixtures come from.
    pub trusted_commit: String,
    /// Digest of the trusted fixture set.
    pub fixture_sha256: String,
    /// Version of the oracle implementation.
    pub version: String,
}

impl Oracle {
    /// Map observed violations to a typed outcome without panicking.
    #[must_use]
    pub fn verdict(&self, violations: &[String]) -> Outcome {
        match violations.len() {
            0 => Outcome::Pass,
            _ => Outcome::Fail,
        }
    }

    /// An oracle cannot be changed inline with the candidate it evaluates
    /// (`ARCHITECTURE.md:55`).
    #[must_use]
    pub const fn may_change_inline() -> Permission {
        Permission::Deny
    }

    /// The attestation stamp this oracle may issue, by name.
    #[must_use]
    pub const fn stamp() -> &'static str {
        "oracle_passed"
    }
}
