//! A signed embargo binding, trusted-only.
//!
//! Mirrors `examples/v61-embargo-rule.example.json` and
//! `schemas/v61-embargo-rule.schema.json`. This value is never a field of a
//! worker-visible type; only its existence may be withheld by omitting content
//! (`docs/20:29-36,57-70`, `docs/28:29-33`).

use super::scope::Scope;
use crate::permission::Permission;
use crate::switch::Switch;
use crate::warning::Warning;

/// The signed embargo rule a trusted broker evaluates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    /// `embargo_id` of the signed object.
    pub embargo_id: String,
    /// `repository_id` scope of the rule.
    pub repository_id: String,
    /// `subject_digest` the rule covers.
    pub subject_digest: String,
    /// `prohibited_recipients` group or holder identifiers.
    pub prohibited_recipients: Vec<String>,
    /// `prohibited_research_tracks` identifiers.
    pub prohibited_research_tracks: Vec<String>,
    /// `prohibited_purposes` such as `publication`.
    pub prohibited_purposes: Vec<String>,
    /// `release_after` earliest allowed UTC instant.
    pub release_after: String,
    /// `issuer_id` of the owner who signed it.
    pub issuer_id: String,
    /// `owner_approved` from the pinned example.
    pub owner_approved: Switch,
    /// `policy_digest` that versioned the rule.
    pub policy_digest: String,
}

impl Rule {
    /// Evaluate the rule against one access scope without revealing a cause.
    #[must_use]
    pub fn applies(&self, scope: &Scope) -> Permission {
        let recipient = self
            .prohibited_recipients
            .iter()
            .any(|item| item == &scope.recipient_id);
        let track = self
            .prohibited_research_tracks
            .iter()
            .any(|item| item == &scope.track_id);
        let purpose = self
            .prohibited_purposes
            .iter()
            .any(|item| item == &scope.purpose);
        let subject = self.subject_digest == scope.subject_digest;
        match subject && (recipient || track || purpose) {
            true => Permission::Deny,
            false => Permission::Allow,
        }
    }

    /// Whether the rule is currently admissible and owner-authorized.
    #[must_use]
    pub fn effective(&self, at: &str) -> Permission {
        Permission::filled(&self.embargo_id)
            .and(Permission::digest(&self.subject_digest))
            .and(Permission::digest(&self.policy_digest))
            .and(self.owner_approved.permit())
            .and(Permission::of(at >= self.release_after.as_str()))
    }

    /// Operator findings for a malformed rule; never worker feedback.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let absent = [
            ("embargo id", self.embargo_id.is_empty()),
            ("repository", self.repository_id.is_empty()),
            ("issuer", self.issuer_id.is_empty()),
            ("release after", self.release_after.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| Warning::Malformed {
                domain: field,
                detail: "an embargo rule must name this field".to_owned(),
            })
        });
        let unapproved = match self.owner_approved {
            Switch::Off => Some(Warning::Withheld {
                subject: "embargo cause",
            }),
            Switch::On => None,
        };
        absent.chain(unapproved).collect()
    }
}
