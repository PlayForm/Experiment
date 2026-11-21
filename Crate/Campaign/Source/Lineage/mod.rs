//! SQLite/WAL event and lineage schema modelled as Rust types.
//!
//! Grounded in all 213 lines of `sql/lineage.sql` (14 tables, five indexes,
//! two immutability triggers) and `REQUIREMENTS.md:718-772` (REQ-LIN-01..10,
//! the minimal proof chain and the append-only ancestry DAG). This is a typed
//! schema, not SQL: no database is opened, no row is inserted, and no event is
//! signed by a generated artifact.
//!
//! The design is illustrative and must be validated and migrated before
//! deployment (`README.md:23-24`, `sql/lineage.sql:1`).

#[path = "Event.rs"]
pub mod event;
#[path = "Table.rs"]
pub mod table;
#[path = "Column.rs"]
pub mod column;
#[path = "Constraint.rs"]
pub mod constraint;
#[path = "Chain.rs"]
pub mod chain;
#[path = "Link.rs"]
pub mod link;
#[path = "Digest.rs"]
pub mod digest;
#[path = "Ancestry.rs"]
pub mod ancestry;
#[path = "Purpose.rs"]
pub mod purpose;

pub use ancestry::Ancestry;
pub use chain::Chain;
pub use column::Column;
pub use constraint::Constraint;
pub use digest::Digest;
pub use event::Event;
pub use link::Link;
pub use purpose::Purpose;
pub use table::Table;
