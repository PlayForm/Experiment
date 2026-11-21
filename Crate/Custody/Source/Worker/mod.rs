//! Offline hidden-worker facade.
//!
//! The worker receives a pinned source snapshot, a task, local Rust tools,
//! private feedback and a scratch directory only. No service host, credential,
//! internal schema or peer correspondence is mounted
//! (`docs/22:1-27`, `docs/28:1-22`).

#[path = "Hidden.rs"]
pub mod hidden;
#[path = "Turn.rs"]
pub mod turn;
#[path = "Correspondence.rs"]
pub mod correspondence;
#[path = "Encrypted.rs"]
pub mod encrypted;
#[path = "LocalTool.rs"]
pub mod local_tool;
#[path = "Scratch.rs"]
pub mod scratch;
