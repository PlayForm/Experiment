//! Structural validation facade.
//!
//! Declares the validator, finding, severity and report leaves. Validation is a
//! typed descriptor operation: a missing reference produces a warning finding
//! and the report continues, never a panic (`docs/26-acceptance-and-build-plan.md:60-80`).
//! Nothing in this domain executes an HTTP call, a JSON Schema engine or a live
//! policy decision.

#[path = "Severity.rs"]
pub mod severity;
#[path = "Finding.rs"]
pub mod finding;
#[path = "Report.rs"]
pub mod report;
#[path = "Validator.rs"]
pub mod validator;

pub use finding::Finding;
pub use report::Report;
pub use severity::Severity;
pub use validator::Validator;
