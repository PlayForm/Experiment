//! Privacy-preserving projections and the audited private mapping boundary.
//!
//! Mirrors `docs/07:1-31`, `docs/02:36-39`, `README.md:14-19` and the
//! private-crosswalk schema. Telemetry exports neutral fields only; a
//! privileged observer gets an audited view, not the private mapping table.
//! No metric is emitted and no crosswalk is decrypted here.

#[path = "Projection.rs"]
pub mod projection;
#[path = "Metadata.rs"]
pub mod metadata;
#[path = "Traffic.rs"]
pub mod traffic;
#[path = "Leakage.rs"]
pub mod leakage;
#[path = "View.rs"]
pub mod view;
#[path = "Audit.rs"]
pub mod audit;
