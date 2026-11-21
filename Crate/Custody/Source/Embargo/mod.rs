//! Embargo policy facade with a strict trusted/worker split.
//!
//! The broker consults a signed rule before context compilation, archive
//! access, research dissemination, publication and transfer. Agents never see
//! it, and a change never retroactively grants an earlier transfer
//! (`docs/20:29-36`).

#[path = "Rule.rs"]
pub mod rule;
#[path = "Scope.rs"]
pub mod scope;
#[path = "Cohort.rs"]
pub mod cohort;
#[path = "Release.rs"]
pub mod release;
#[path = "Reason.rs"]
pub mod reason;
