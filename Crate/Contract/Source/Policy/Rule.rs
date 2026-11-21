//! Explicit allow/deny rule for one role.
//!
//! Every row of `policy/access-matrix.json:46-182` is copied as typed action
//! lists. `policy/v61-roles.json:68-70` fixes the decision order: explicit deny
//! overrides explicit permit, and anything else falls to default deny.

use crate::policy::Role;

/// One role's authorization rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rule {
    /// Subject role.
    pub role: Role,
    /// Wire role name.
    pub name: &'static str,
    /// Explicitly permitted actions.
    pub allow: &'static [&'static str],
    /// Explicitly denied actions.
    pub deny: &'static [&'static str],
}

impl Rule {
    /// Build one role rule.
    #[must_use]
    pub const fn of(
        role: Role,
        name: &'static str,
        allow: &'static [&'static str],
        deny: &'static [&'static str],
    ) -> Self {
        Self { role, name, allow, deny }
    }

    /// All twelve role rules.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
        Rule::of(Role::Observer, "observer", &["read-public-projections", "subscribe-sanitized-events"], &["write", "read-prompt-content", "read-private-cas", "approve", "issue-identity"]),
        Rule::of(Role::Operator, "operator", &["read-projections", "submit-predeclared-command"], &["write-ledger-direct", "modify-trusted-evidence", "read-private-prompt-unless-explicit-grant"]),
        Rule::of(Role::Coordinator, "coordinator", &["transition-state", "schedule-build", "schedule-oracle", "schedule-benchmark", "publish-metadata"], &["sign-measurement", "decrypt-private-agent-reasoning", "run-agent-code", "alter-policy"]),
        Rule::of(Role::Supervisor, "supervisor", &["prepare-scope", "mount-task", "run-model-locally", "seal-conversation", "submit-worker-artifact"], &["sign-benchmark", "publish", "access-holdout", "change-policy"]),
        Rule::of(Role::Worker, "llm-worker", &["read-mounted-task", "write-scratch-output", "invoke-local-scoped-tools"], &["network", "docker-socket", "host-sockets", "read-credentials", "read-telemetry", "read-ledger", "read-sibling", "read-holdout", "schedule-job", "approve-evidence", "change-policy"]),
        Rule::of(Role::Builder, "builder", &["build-from-pinned-source", "seal-build-result"], &["alter-oracle", "select-benchmark-winner", "read-private-conversations"]),
        Rule::of(Role::Oracle, "oracle", &["execute-trusted-tests", "seal-test-result"], &["modify-tests", "read-agent-private-context", "publish-result-as-release"]),
        Rule::of(Role::Runner, "runner", &["lease-exclusive-host", "measure-frozen-binary", "seal-raw-samples"], &["modify-source", "read-private-conversations", "edit-statistical-policy"]),
        Rule::of(Role::Evaluator, "evaluator", &["read-authorized-raw-samples", "apply-frozen-statistical-protocol", "sign-evaluation"], &["edit-samples", "promote-source", "modify-benchmark"]),
        Rule::of(Role::Publisher, "publisher", &["access-approved-submissions", "write-sanitized-derivative"], &["read-unsubmitted-notebooks", "reclassify-measurement", "auto-release-public"]),
        Rule::of(Role::PKI, "pki", &["issue-scoped-certificates", "revoke", "append-issuance-audit"], &["sign-scientific-evidence", "decrypt-private-messages"]),
        Rule::of(Role::Telemetry, "telemetry", &["read-metadata", "store-approved-metrics", "store-approved-spans"], &["read-plaintext-reasoning", "promote-result", "change-policy"]),
        ]
    }

    /// Decide an action with deny overriding permit, then default deny.
    #[must_use]
    pub fn decide(&self, action: &str) -> crate::Permission {
        match (
            self.deny.iter().any(|denied| *denied == action),
            self.allow.iter().any(|allowed| *allowed == action),
        ) {
            (true, _) | (false, false) => crate::Permission::Deny,
            (false, true) => crate::Permission::Allow,
        }
    }
}
