//! The bounded loop engine: one atomic mutation per cycle, bounded by the
//! playbook budget, with typed stop and continue signals. Grounded in
//! `ARCHITECTURE.md:98-109` (the append-only state machine),
//! `:131-142` (finite depth, attempt cap, wall clock and watchdog) and
//! `18-emergent-evolution.md:21-39` (atomicity and independence).

#[path = "Budget.rs"]
pub mod budget;
#[path = "Cycle.rs"]
pub mod cycle;
#[path = "Iteration.rs"]
pub mod iteration;
#[path = "Generation.rs"]
pub mod generation;
#[path = "Stop.rs"]
pub mod stop;
// File name `Continue` is mandated by the card; `continue` is a Rust keyword,
// so the module is reached through the raw identifier `r#continue`.
#[path = "Continue.rs"]
pub mod r#continue;
