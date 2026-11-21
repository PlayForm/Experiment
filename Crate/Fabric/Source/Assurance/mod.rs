//! Assurance plan, threat model, negative cases and explicit limitations.
//!
//! Mirrors `docs/09:1-89`, `README.md:3-6`, `VALIDATION_REPORT.md:14-20` and
//! the honest evidence ladder in `Status.rs`. A finding records what was
//! reasoned about; it never fabricates a passing test, an audit or a
//! cryptographic proof.

#[path = "Threat.rs"]
pub mod threat;
#[path = "Negative.rs"]
pub mod negative;
#[path = "Integration.rs"]
pub mod integration;
#[path = "Limit.rs"]
pub mod limit;
#[path = "Finding.rs"]
pub mod finding;
