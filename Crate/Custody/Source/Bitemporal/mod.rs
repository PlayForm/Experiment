//! Bitemporal two-timescale propagation facade.
//!
//! Fast private feedback stays in the worker's own track; slow sanctioned
//! knowledge crosses repositories only after rights, embargo and confidentiality
//! checks and a new experiment on the recipient. Corrections supersede earlier
//! facts instead of mutating them
//! (`docs/21:3-46`, `docs/17:51`).

#[path = "Fact.rs"]
pub mod fact;
#[path = "Valid.rs"]
pub mod valid;
#[path = "Recorded.rs"]
pub mod recorded;
#[path = "Epoch.rs"]
pub mod epoch;
#[path = "Supersede.rs"]
pub mod supersede;
