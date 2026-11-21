//! One signed, immutable schema/policy migration transition.
//!
//! Mirrors all eleven fields of `schemas/migration-transition.schema.json`:
//! migration_id, previous_head, schema_before, schema_after, policy_after,
//! compiler_digest, epoch_before, epoch_after, phase, approver and signature.
//! `docs/09:73-76` requires `epoch_after > epoch_before`; no step is applied.

use crate::verdict::Verdict;
use crate::stage::Stage;

/// One recorded migration transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Migration {
    /// Opaque migration identifier.
    pub migration_id: String,
    /// Journal head before the migration.
    pub previous_head: String,
    /// Digest of the schema before the migration.
    pub schema_before: String,
    /// Digest of the schema after the migration.
    pub schema_after: String,
    /// Digest of the policy after the migration.
    pub policy_after: String,
    /// Digest of the compiler that produced the bundle.
    pub compiler_digest: String,
    /// Epoch before the migration.
    pub epoch_before: u64,
    /// Epoch after the migration.
    pub epoch_after: u64,
    /// Signed phase of the transition.
    pub phase: Stage,
    /// Opaque approver identifier.
    pub approver: String,
    /// Signature over the canonical transition.
    pub signature: String,
}

impl Migration {
    /// Record a transition from trusted fields.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn of(
        migration_id: &str,
        previous_head: &str,
        schema_before: &str,
        schema_after: &str,
        policy_after: &str,
        compiler: &str,
        epoch_before: u64,
        epoch_after: u64,
        phase: Stage,
        approver: &str,
        signature: &str,
    ) -> Self {
        Self {
            migration_id: migration_id.to_owned(),
            previous_head: previous_head.to_owned(),
            schema_before: schema_before.to_owned(),
            schema_after: schema_after.to_owned(),
            policy_after: policy_after.to_owned(),
            compiler_digest: compiler.to_owned(),
            epoch_before,
            epoch_after,
            phase,
            approver: approver.to_owned(),
            signature: signature.to_owned(),
        }
    }

    /// Validate the signed transition shape and the monotonic-epoch rule.
    #[must_use]
    pub fn valid(&self) -> Verdict {
        Verdict::filled(&self.migration_id)
            .and(Verdict::digest(&self.previous_head))
            .and(Verdict::digest(&self.schema_before))
            .and(Verdict::digest(&self.schema_after))
            .and(Verdict::digest(&self.policy_after))
            .and(Verdict::digest(&self.compiler_digest))
            .and(Verdict::filled(&self.approver))
            .and(Verdict::filled(&self.signature))
            .and(Verdict::of(self.epoch_after > self.epoch_before))
    }

    /// Whether the transition has been atomically activated.
    #[must_use]
    pub const fn activated(&self) -> bool {
        self.phase.active()
    }
}
