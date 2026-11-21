//! Candidate patches as typed data. When materialised, a candidate is written
//! only as a `.rs` file under `Crate/Loop/Source/Candidates/`; nothing here
//! writes a file, executes generated code or touches `Module/Run`.

#[path = "Proposal.rs"]
pub mod proposal;
#[path = "Diff.rs"]
pub mod diff;
#[path = "RustFile.rs"]
pub mod rust_file;
#[path = "Origin.rs"]
pub mod origin;
