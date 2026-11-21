//! Command dispatch modules. `loop` is a keyword, so its Rust module name is
//! `loop_` while the physical file and the user-facing word stay `Loop`.

#[path = "Plan.rs"]
pub mod plan;
#[path = "Bench.rs"]
pub mod bench;
#[path = "Loop.rs"]
pub mod loop_;
