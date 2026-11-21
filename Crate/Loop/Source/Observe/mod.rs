//! Read-only observation of the SUT at `Module/Run`. Nothing here writes into
//! that submodule. Grounded in `22-hidden-worker-and-correspondence.md:1-28`
//! (the single visible pinned snapshot at `/workspace/repo`) and
//! `ARCHITECTURE.md:127-129` (build and benchmark source are immutable).

#[path = "Snapshot.rs"]
pub mod snapshot;
#[path = "Baseline.rs"]
pub mod baseline;
#[path = "Holdout.rs"]
pub mod holdout;
#[path = "Contract.rs"]
pub mod contract;
#[path = "Reading.rs"]
pub mod reading;
