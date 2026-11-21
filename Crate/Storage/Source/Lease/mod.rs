//! Exclusive benchmark and builder lease facade.
//!
//! A lease carries a monotonically increasing fence; an expired or revoked
//! holder cannot produce authoritative output and a successor cannot reuse
//! the old authority token (`docs/14-runtime-and-recovery.md:30`,
//! `sql/v6_migration.sql:170-183`, `sql/v61_migration.sql:145-160`).

#[path = "Role.rs"]
pub mod role;
#[path = "Owner.rs"]
pub mod owner;
#[path = "State.rs"]
pub mod state;
#[path = "Expiry.rs"]
pub mod expiry;
#[path = "Renewal.rs"]
pub mod renewal;
#[path = "ReleaseReason.rs"]
pub mod release_reason;
#[path = "Release.rs"]
pub mod release;
#[path = "Lease.rs"]
pub mod lease;
