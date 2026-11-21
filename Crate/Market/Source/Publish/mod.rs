//! The explicit declassification boundary of the research economy.
//!
//! Grounded in `PUBLISHING.md:42-70` (a privileged but non-omniscient publisher,
//! sensitive scans, human review of uncertain cases, and a distinct issuer for
//! the release grant), `SYSTEM.md:56,67-70` (a sanitized derivative with
//! provenance links; notebooks never connect directly to the library),
//! `V4_TRUST_BRIDGE.md:37-47` (authorized recipient, content-addressed public
//! hash, private link in an access-controlled ledger) and
//! `sql/v5_migration.sql:230-240` (signed publication release). Nothing is
//! scanned, decrypted, sanitized, stamped or released here.

#[path = "Sanitizer.rs"]
pub mod sanitizer;
#[path = "Declassification.rs"]
pub mod declassification;
#[path = "Stamp.rs"]
pub mod stamp;
#[path = "Provenance.rs"]
pub mod provenance;

pub use declassification::Declassification;
pub use provenance::Provenance;
pub use sanitizer::Sanitizer;
pub use stamp::Stamp;
