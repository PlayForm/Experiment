//! The fifteen security-critical negative tests as typed cases.
//!
//! Mirrors the numbered list in `docs/09:16-51`. Each case is a planned
//! negative check, never a claim that it was executed. No test is generated or
//! run by this crate.

/// One planned security-critical negative case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Negative {
    /// 1. Invalid identifiers must be rejected by parser and schema alike.
    InvalidIdentifier,
    /// 2. Canonicalization and URI-reference traversal must be rejected.
    ReferenceTraversal,
    /// 3. A mocked generator collision must abort reservation.
    ForgedCollision,
    /// 4. Pseudonyms across audiences and epochs must be unlinkable.
    CrossAudiencePseudonym,
    /// 5. Swapped ciphertext, context, nonce or schema epoch must fail.
    SwappedCiphertext,
    /// 6. A valid grant presented by the wrong workload must be rejected.
    WrongWorkload,
    /// 7. Expired, replayed or revoked grants never authorize.
    ExpiredGrant,
    /// 8. Rollback, reorder, truncation, double activation or concurrency.
    MigrationAttack,
    /// 9. Injected metacharacters as schema names must not execute.
    InjectedName,
    /// 10. A worker with no network must have no egress path.
    WorkerEgress,
    /// 11. Telemetry planes and public aggregates must not reveal mappings.
    TelemetryReach,
    /// 12. Expired leases and revoked rights prevent new access only.
    RevokedLease,
    /// 13. Sealed-backup restore must recover exact lineage and key epochs.
    CrashRestore,
    /// 14. Candidate patches must not author trusted evidence.
    CandidateEvidence,
    /// 15. Boundary failures are logged without revealing hidden policy.
    BoundaryFailure,
}

impl Negative {
    /// All fifteen cases in plan order.
    #[must_use]
    pub const fn all() -> [Self; 15] {
        [
            Self::InvalidIdentifier,
            Self::ReferenceTraversal,
            Self::ForgedCollision,
            Self::CrossAudiencePseudonym,
            Self::SwappedCiphertext,
            Self::WrongWorkload,
            Self::ExpiredGrant,
            Self::MigrationAttack,
            Self::InjectedName,
            Self::WorkerEgress,
            Self::TelemetryReach,
            Self::RevokedLease,
            Self::CrashRestore,
            Self::CandidateEvidence,
            Self::BoundaryFailure,
        ]
    }

    /// Stable audit identifier.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "invalid-identifier",
            Self::ReferenceTraversal => "reference-traversal",
            Self::ForgedCollision => "forged-collision",
            Self::CrossAudiencePseudonym => "cross-audience-pseudonym",
            Self::SwappedCiphertext => "swapped-ciphertext",
            Self::WrongWorkload => "wrong-workload",
            Self::ExpiredGrant => "expired-grant",
            Self::MigrationAttack => "migration-attack",
            Self::InjectedName => "injected-name",
            Self::WorkerEgress => "worker-egress",
            Self::TelemetryReach => "telemetry-reach",
            Self::RevokedLease => "revoked-lease",
            Self::CrashRestore => "crash-restore",
            Self::CandidateEvidence => "candidate-evidence",
            Self::BoundaryFailure => "boundary-failure",
        }
    }

    /// A planned case is not an observed pass.
    #[must_use]
    pub const fn is_evidence() -> bool {
        false
    }
}
