//! Storage: append-only ledger plus content-addressed artifacts.
//!
//! Grounded in `STORAGE.md:1-95`, `REQUIREMENTS_V3.md` DAT-001..008,
//! `sql/v3_migration.sql` and `ARCHITECTURE.md:9-10`. This is the typed model
//! of the v3 storage contract; the programme's dedicated `Crate/Storage` owns
//! the executable implementation (a coupling recorded as a typed name).

#[path = "Store.rs"]
pub mod store;
#[path = "Ledger.rs"]
pub mod ledger;
#[path = "Cas.rs"]
pub mod cas;
#[path = "Index.rs"]
pub mod index;
#[path = "Migration.rs"]
pub mod migration;
