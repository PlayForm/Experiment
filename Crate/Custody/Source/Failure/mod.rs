//! Thirty typed failure cases with honest disclosure facade.
//!
//! No fabricated passing test, no misleading success notice and no fabricated
//! benchmark result; a negative outcome is a first-class value
//! (`docs/25:3-34`, `docs/28:34-37`, `docs/26:82-89`).

#[path = "Case.rs"]
pub mod case;
#[path = "Severity.rs"]
pub mod severity;
#[path = "Cause.rs"]
pub mod cause;
#[path = "Disclosure.rs"]
pub mod disclosure;
#[path = "Recovery.rs"]
pub mod recovery;
