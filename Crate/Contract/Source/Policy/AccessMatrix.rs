//! The versioned, pinned authorization matrix.
//!
//! Combines `policy/access-matrix.json` (revision 6: six trust domains, twelve
//! role rules, ten principles) with `policy/v61-roles.json` (revision 6.1:
//! model trust domain with no routes, 18 service grants, untrusted
//! and trusted-only artifact classes, priority and telemetry exclusions).
//! Deny is the default; the matrix is data, not a live decision point.

use crate::policy::{Domain, Role, Rule, Service};

/// The complete pinned authorization matrix.
pub struct AccessMatrix {
    /// Base revision.
    pub version: &'static str,
    /// v6.1 revision.
    pub revision: &'static str,
    /// Default decision when nothing explicitly permits.
    pub policy_default: crate::Permission,
    /// Declared trust domains.
    pub domains: Vec<Domain>,
    /// Declared role rules.
    pub rules: Vec<Rule>,
    /// Service route grants.
    pub services: Vec<Service>,
    /// Authorization principles.
    pub principles: &'static [&'static str],
    /// Artifacts that may only come from untrusted model output.
    pub untrusted_output: &'static [&'static str],
    /// Artifacts that trusted authority alone may issue.
    pub trusted_authority_only: &'static [&'static str],
    /// Decision priority statement.
    pub priority: &'static str,
    /// Treatment of unknown policy terms.
    pub unknown_terms: &'static str,
    /// Reader class for the contract projections.
    pub reader_class: &'static str,
    /// Attributes excluded from all telemetry.
    pub telemetry_exclude: &'static [&'static str],
}

impl AccessMatrix {
    /// Build the canonical pinned matrix.
    #[must_use]
    pub fn of() -> Self {
        Self {
            version: "6",
            revision: "6.1",
            policy_default: crate::Permission::Deny,
            domains: Domain::all(),
            rules: Rule::all(),
            services: Service::all(),
            principles: &["default-deny", "auth-before-payload-parse", "scoped-service-identity", "idempotent-write", "fenced-lease", "no-LLM-trust", "no-key-shares-to-workers", "no-control-api-in-LLM-namespaces", "no-public-listen-by-default", "strict-artifact-grants"],
            untrusted_output: &["patch", "hypothesis", "test-proposal", "draft-paper"],
            trusted_authority_only: &["experiment-spec", "custody-decision", "custody-receipt", "oracle-result", "benchmark-result", "evaluation-result", "knowledge-release"],
            priority: "explicit-deny > explicit-permit > default-deny",
            unknown_terms: "deny_and_request_review",
            reader_class: "operator/observer only",
            telemetry_exclude: &["prompt_text", "source_content", "private_correspondence", "embargo_targets", "trade_edges", "secret_paths", "private_key", "model_private_thought", "raw_license_text"],
        }
    }

    /// Look up one role rule.
    #[must_use]
    pub fn rule(&self, role: Role) -> Option<&Rule> {
        self.rules.iter().find(|rule| rule.role == role)
    }

    /// The model trust domain has no routes, credentials, APIs or transitions.
    #[must_use]
    pub fn model_has_no_routes() -> crate::Permission {
        crate::Permission::Deny
    }
}
