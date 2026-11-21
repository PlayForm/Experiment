//! Hyperfine JSON schema v2 measurements, summaries and version gate.
//!
//! Grounded in `README.md:22-24,132-134` and `prun_lab.py::report` (600-643):
//! the CSV report reads `results[].summary.time_wall_clock.mean` and
//! `results[].measurements`, and rejects any `schema_version` other than 2.

#[path = "Sample.rs"]
pub mod sample;
#[path = "Measurement.rs"]
pub mod measurement;
#[path = "Summary.rs"]
pub mod summary;
#[path = "SchemaVersion.rs"]
pub mod schema_version;

#[path = "Clock.rs"]
pub mod clock;
#[path = "Observation.rs"]
pub mod observation;
#[path = "Document.rs"]
pub mod document;

pub use clock::Clock;
pub use document::Document;
pub use measurement::Measurement;
pub use observation::Observation;
pub use sample::Sample;
pub use schema_version::SchemaVersion;
pub use summary::Summary;
