//! CSV summaries, build metadata and per-session retention paths.
//!
//! Grounded in `README.md:59-60,132-134` and `prun_lab.py:437-459,532-536,
//! 600-643`. Reports are typed records and rendered strings; no files are
//! written by this crate.

#[path = "Row.rs"]
pub mod row;
#[path = "Csv.rs"]
pub mod csv;
#[path = "Metadata.rs"]
pub mod metadata;
#[path = "Session.rs"]
pub mod session;

#[path = "Host.rs"]
pub mod host;
#[path = "ImageRecord.rs"]
pub mod image_record;

pub use csv::Csv;
pub use host::Host;
pub use image_record::ImageRecord;
pub use metadata::Metadata;
pub use row::Row;
pub use session::Session;
