//! The directory walk and path-selection contract.
//!
//! Grounded in `Fn/Binary/Command/Entry.rs:29-52` (`WalkDir`,
//! `follow_links(false)`, `GlobSet` exclusion, `filter_map(Result::ok)`,
//! the `-F` file filter) and `README.md:118,148-149,166-179`.
//! Matching is a pure segment/wildcard model of `globset`: no filesystem is
//! read and no process is spawned.

#[path = "Walker.rs"]
pub mod walker;
#[path = "Entry.rs"]
pub mod entry;
#[path = "Glob.rs"]
pub mod glob;
#[path = "Exclusion.rs"]
pub mod exclusion;
#[path = "Symlink.rs"]
pub mod symlink;
