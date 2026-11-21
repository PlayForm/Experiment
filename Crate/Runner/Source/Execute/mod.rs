//! Execution strategies, process spawn and lock contracts.
//!
//! Grounded in every file under `Fn/Binary/Command/**` (including
//! `Parallel/GPG.rs` and `Parallel/GPG/Git.rs`) and read-only: no process is
//! spawned, no lock file is touched, no Tokio or Rayon runtime starts.

#[path = "Sequential.rs"]
pub mod sequential;
#[path = "Parallel.rs"]
pub mod parallel;
#[path = "Process.rs"]
pub mod process;
#[path = "Spawn.rs"]
pub mod spawn;
#[path = "Index.rs"]
pub mod index;
#[path = "Lock.rs"]
pub mod lock;
#[path = "Gpg.rs"]
pub mod gpg;
#[path = "Git.rs"]
pub mod git;
