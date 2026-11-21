//! Fast in-memory projections rebuilt from encrypted, signed durable state.
//!
//! Mirrors `docs/06:1-49`, `docs/01:27-33,41-48`, `docs/08:43-50` and the
//! `audit-commitment` and `migration-transition` schemas. Arenas are temporary
//! allocation aids, never vaults; the only immutable history is the encrypted,
//! signed, append-only journal. No store is opened and no fault is injected.

#[path = "Arena.rs"]
pub mod arena;
#[path = "Journal.rs"]
pub mod journal;
#[path = "Entry.rs"]
pub mod entry;
#[path = "Epoch.rs"]
pub mod epoch;
#[path = "Checkpoint.rs"]
pub mod checkpoint;
#[path = "Migration.rs"]
pub mod migration;
#[path = "Recovery.rs"]
pub mod recovery;
