//! Crash-recovery facade over immutable events.
//!
//! In-memory projections are never the only copy: the signed journal and its
//! checkpoints survive a crash, and a missed outbox message is repaired by
//! replay from the ledger sequence (`docs/14-runtime-and-recovery.md:24-40`).

#[path = "Outcome.rs"]
pub mod outcome;
#[path = "Epoch.rs"]
pub mod epoch;
#[path = "EpochState.rs"]
pub mod epoch_state;
#[path = "Checkpoint.rs"]
pub mod checkpoint;
#[path = "Divergence.rs"]
pub mod divergence;
#[path = "Scenario.rs"]
pub mod scenario;
#[path = "Replay.rs"]
pub mod replay;
