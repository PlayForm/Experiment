//! An immutable, signed, idempotent resource transaction.
//!
//! Grounded in `sql/v5_migration.sql:119-149` (causal transaction ID, unique
//! request idempotency key, kind, seven native amounts, policy and evidence
//! digests, issuer identity and signature) and `MARKET.md:70-79` (signed actor
//! and rationale; trusted metering, not agent-reported spend). A transaction
//! here is validated data; no balance is changed and no signature is verified.

use crate::{Digest, Permission, Resource, Warning};

use super::Ledger;

/// One append-only treasury transaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    /// Causal transaction identifier (`sql:122`).
    pub transaction_id: String,
    /// Unique idempotency key (`sql:126`).
    pub request_id: String,
    /// Transaction kind (`sql:127-136`).
    pub kind: Ledger,
    /// Native-unit amounts (`sql:137-143`).
    pub amounts: Resource,
    /// Versioned policy digest (`sql:144`).
    pub policy_sha256: Digest,
    /// Evidence digest for the rationale (`sql:145`).
    pub evidence_sha256: Digest,
    /// Signing actor (`sql:146`).
    pub issuer_identity: String,
    /// Signature artifact digest (`sql:147`).
    pub signature_sha256: Digest,
}

impl Transaction {
    /// Validate the two required identifiers; a duplicate is a replay warning.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.transaction_id.is_empty(),
            self.request_id.is_empty(),
            self.issuer_identity.is_empty(),
        ) {
            (false, false, false) => Ok(()),
            (true, _, _) => Err(Warning::InvalidValue {
                field: "transaction_id",
                detail: "a causal transaction ID is required".to_owned(),
            }),
            (_, true, _) => Err(Warning::InvalidValue {
                field: "request_id",
                detail: "an idempotency key is required".to_owned(),
            }),
            (_, _, true) => Err(Warning::Denied {
                subject: "transaction",
                reason: "missing signed issuer",
            }),
        }
    }

    /// Reject a replayed idempotency key (`sql:126`, `V4_TRUST_BRIDGE.md:48`).
    pub fn check_replay(&self, seen: bool) -> Result<(), Warning> {
        self.validate()?;
        match seen {
            true => Err(Warning::Replay {
                message_id: self.request_id.clone(),
            }),
            false => Ok(()),
        }
    }

    /// Ledger rows are immutable (`sql:267-278`).
    #[must_use]
    pub const fn immutable() -> Permission {
        Permission::Allow
    }
}
