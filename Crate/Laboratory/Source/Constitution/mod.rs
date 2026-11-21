//! Constitutional rules as typed values.
//!
//! This facade declares only the leaves; there is no behaviour here. Every rule
//! in the domain applies crate-wide through these types rather than as a prose
//! note beside one inspected path (`docs/00-system-constitution.md`,
//! `docs/17-v61-constitution.md`).

#[path = "Invariant.rs"]
pub mod invariant;
#[path = "Prohibition.rs"]
pub mod prohibition;
#[path = "Domain.rs"]
pub mod domain;
#[path = "Revision.rs"]
pub mod revision;

pub use domain::Domain;
pub use invariant::Invariant;
pub use prohibition::Prohibition;
pub use revision::Revision;
