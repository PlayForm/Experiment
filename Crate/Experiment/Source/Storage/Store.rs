//! Append-only ledger plus content-addressed artifacts and rebuildable index.
//!
//! Grounded in `STORAGE.md:1-37` (authoritative domains, filesystem layout),
//! `REQUIREMENTS_V3.md` DAT-001/DAT-002 and `sql/v3_migration.sql:1-3,191-229`
//! (transactional appends, outbox, immutable audit tables). The event outbox
//! ensures a state change cannot commit without a durable dispatch intent.

use crate::interface::envelope::Envelope;
use crate::storage::cas::Cas;
use crate::storage::index::Index;
use crate::storage::ledger::Ledger;
use crate::warning::Warning;

/// The durable store facade used by the deterministic coordinator.
#[derive(Clone, Debug, Default)]
pub struct Store {
    /// The authoritative append-only event ledger.
    pub ledger: Ledger,
    /// The content-addressed immutable artifact store.
    pub cas: Cas,
    /// The rebuildable query/lineage projection.
    pub index: Index,
}

impl Store {
    /// Append an envelope after the ledger has verified its sequence.
    pub fn append(&mut self, event: Envelope) -> Result<(), Warning> {
        self.ledger.append(event)
    }

    /// Publish an artifact hash as an immutable CAS object.
    pub fn publish(&mut self, digest: String) -> Result<(), Warning> {
        self.cas.publish(digest)
    }

    /// Rebuild a projection from the authoritative records
    /// (`STORAGE.md:34-37`).
    pub fn rebuild(&mut self) {
        self.index = Index::from_ledger(&self.ledger);
    }

    /// Whether any dashboard projection could be used as primary truth. It
    /// never can (`REQUIREMENTS_V3.md` DAT-001).
    #[must_use]
    pub const fn projection_is_authoritative() -> bool {
        false
    }
}
