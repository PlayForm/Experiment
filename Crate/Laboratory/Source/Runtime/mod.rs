//! Runtime and recovery facade.
//!
//! Declares only the four required leaves. Crash-recoverable state is derived
//! from signed append-only events; an irreconcilable outcome stays inconclusive
//! or quarantined rather than becoming an automatic success
//! (`docs/14-runtime-and-recovery.md:24-40`).

#[path = "Recovery.rs"]
pub mod recovery;
#[path = "Lease.rs"]
pub mod lease;
#[path = "Crash.rs"]
pub mod crash;
#[path = "Mirror.rs"]
pub mod mirror;

pub use crash::Crash;
pub use lease::Lease;
pub use mirror::Mirror;
pub use recovery::Recovery;
