//! Telemetry: coordinator/event signal plane, separate from product
//! measurement and from the authoritative event ledger.
//!
//! Grounded in `TELEMETRY.md:1-171`, `REQUIREMENTS_V3.md` TEL-001..012,
//! `telemetry/otel-collector-optional.yaml` and `DASHBOARD.md:21-41`.

#[path = "Plane.rs"]
pub mod plane;
#[path = "Signal.rs"]
pub mod signal;
#[path = "Index.rs"]
pub mod index;
#[path = "Ingest.rs"]
pub mod ingest;
#[path = "Redaction.rs"]
pub mod redaction;
