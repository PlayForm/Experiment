//! The program entry chain and the four bin aliases.
//!
//! Grounded in `Source/Library.rs:1-17` (the `#[tokio::main]` entry and the
//! `(Fn().Fn)().await` expression) and `Cargo.toml:1-15,47` (the four `[[bin]]`
//! aliases and `default-run`).

#[path = "Entry.rs"]
pub mod entry;
#[path = "Fn.rs"]
pub mod function;
#[path = "Alias.rs"]
pub mod alias;
