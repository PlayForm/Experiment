//! The additive trusted Rust IPC operations, never worker HTTP.
//!
//! Mirrors the thirteen operations in `docs/08:22-51`: Registry.PrepareDefinition,
//! Registry.ActivateDefinition, Resolver.Resolve, Pseudonym.Issue,
//! Capability.Issue, Capability.Check, Custody.Authorize, KeyBroker.Seal,
//! KeyBroker.Open, Journal.Append, Migration.Preview, Migration.Commit and
//! Audit.Project. All are proposed, not implemented (`docs/08:52-56`).

/// One named trusted control operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Api {
    /// Validate signed canonical IR and closure, no activation.
    PrepareDefinition,
    /// Atomically activate a staged definition.
    ActivateDefinition,
    /// Policy-enforced and audited opaque-handle resolution.
    Resolve,
    /// Issue an audience/epoch scoped pseudonym.
    IssuePseudonym,
    /// Issue a signed, proof-bound capability.
    IssueCapability,
    /// Check a grant with a fresh nonce; fail closed.
    CheckCapability,
    /// Check holder capability, license, embargo, quota and key scope.
    AuthorizeCustody,
    /// Seal plaintext to a recipient handle; no key export.
    Seal,
    /// Open ciphertext only for a preauthorized recipient endpoint.
    Open,
    /// Compare-and-swap append of an encrypted event.
    AppendJournal,
    /// Dry-run a signed migration with negative checks.
    PreviewMigration,
    /// Atomically commit a migration after policy and compatibility.
    CommitMigration,
    /// Return redacted records and audit the mapping request.
    ProjectAudit,
}

impl Api {
    /// All thirteen additive operations in declaration order.
    #[must_use]
    pub const fn all() -> [Self; 13] {
        [
            Self::PrepareDefinition,
            Self::ActivateDefinition,
            Self::Resolve,
            Self::IssuePseudonym,
            Self::IssueCapability,
            Self::CheckCapability,
            Self::AuthorizeCustody,
            Self::Seal,
            Self::Open,
            Self::AppendJournal,
            Self::PreviewMigration,
            Self::CommitMigration,
            Self::ProjectAudit,
        ]
    }

    /// No control operation is callable by an LLM worker
    /// (`docs/08:52-56`).
    #[must_use]
    pub const fn worker_callable(&self) -> bool {
        false
    }

    /// Stable dotted wire name from `docs/08:22-51`.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::PrepareDefinition => "Registry.PrepareDefinition",
            Self::ActivateDefinition => "Registry.ActivateDefinition",
            Self::Resolve => "Resolver.Resolve",
            Self::IssuePseudonym => "Pseudonym.Issue",
            Self::IssueCapability => "Capability.Issue",
            Self::CheckCapability => "Capability.Check",
            Self::AuthorizeCustody => "Custody.Authorize",
            Self::Seal => "KeyBroker.Seal",
            Self::Open => "KeyBroker.Open",
            Self::AppendJournal => "Journal.Append",
            Self::PreviewMigration => "Migration.Preview",
            Self::CommitMigration => "Migration.Commit",
            Self::ProjectAudit => "Audit.Project",
        }
    }
}
