//! Additive-only migration facade.
//!
//! The chain is v2 -> v3 -> v5 -> v6 -> v6.1; no step rewrites an earlier
//! table. A destructive step becomes a warning-level [`Finding`], never an
//! applied change (`sql/v61_migration.sql:1-2`, `docs/08:70-80`).

#[path = "Migration.rs"]
pub mod migration;
#[path = "Version.rs"]
pub mod version;
#[path = "Additive.rs"]
pub mod additive;
#[path = "Change.rs"]
pub mod change;
#[path = "Step.rs"]
pub mod step;
#[path = "Finding.rs"]
pub mod finding;
#[path = "Guard.rs"]
pub mod guard;
#[path = "Chain.rs"]
pub mod chain;
