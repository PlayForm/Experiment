//! Append-only event ledger facade.
//!
//! The journal is canonical: every authoritative transition appends one signed
//! event with a global sequence, a per-aggregate sequence and a hash chain
//! (`sql/v6_migration.sql:4-34`, `docs/08-data-and-storage.md:5-8`).
//! In-memory projections are rebuilt from it, never trusted as the only copy.

#[path = "Privacy.rs"]
pub mod privacy;
#[path = "Event.rs"]
pub mod event;
#[path = "Sequence.rs"]
pub mod sequence;
#[path = "Append.rs"]
pub mod append;
#[path = "Cursor.rs"]
pub mod cursor;
#[path = "Replay.rs"]
pub mod replay;
