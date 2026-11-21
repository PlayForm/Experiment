//! Signed trade/conflict graph with non-disclosing enforcement facade.
//!
//! Default is no lateral trade; a denial never reveals peer identity or the
//! rule that fired (`docs/18:45-53`, `docs/20:38-70`).

#[path = "Edge.rs"]
pub mod edge;
#[path = "Denied.rs"]
pub mod denied;
#[path = "Reason.rs"]
pub mod reason;
#[path = "Jurisdiction.rs"]
pub mod jurisdiction;
