//! Clean-room, model-only re-synthesis of the `Module/Run` binary (package
//! `prun` v0.3.0), the SYSTEM UNDER TEST that the whole laboratory benchmarks.
//!
//! Every item here is typed data or a pure function. Nothing spawns a process,
//! walks a real filesystem, starts a runtime, or drives a terminal. This crate
//! is the CLI/semantics contract the evidence oracle measures against; it is
//! an untrusted proposal and never signs a measurement or changes policy.
//!
//! Grounding: `Module/Run/Cargo.toml`, `.cargo/config.toml`, `README.md`,
//! `build.rs`, and every file under `Module/Run/Source/`. Each leaf carries
//! its own file-and-line citation in its module doc comment.
//!
//! On this case-sensitive volume the physical directories are PascalCase, so
//! every Rust module is declared with an explicit `#[path]` attribute and a
//! lowercase Rust identifier.

#![forbid(unsafe_code)]
// `GPG` is a genuine acronym and stays fully cased (house style).
#![allow(clippy::upper_case_acronyms)]

#[path = "Fault.rs"]
pub mod fault;

#[path = "Cli/mod.rs"]
pub mod cli;
#[path = "Walk/mod.rs"]
pub mod walk;
#[path = "Execute/mod.rs"]
pub mod execute;
#[path = "Event/mod.rs"]
pub mod event;
#[path = "Tui/mod.rs"]
pub mod tui;
#[path = "Binary/mod.rs"]
pub mod binary;
#[path = "Contract/mod.rs"]
pub mod contract;

/// The SUT package name (`Cargo.toml:59`).
pub const PACKAGE: &str = "prun";
/// The SUT version (`Cargo.toml:60`; injected by `build.rs:11-20`).
pub const VERSION: &str = "0.3.0";
/// The default exclusion glob (`README.md:118`, `Command.rs:54`).
pub const DEFAULT_EXCLUDE: &str = "**/{node_modules,.git,target,dist,vendor}/**/*";
