//! The exact clap surface of the SUT.
//!
//! Grounded in `Fn/Binary/Command.rs` (the `Command::new("Run")` builder and
//! its seven `Arg`s), `Struct/Binary/Command/Option.rs` (the parsed value
//! struct and its defaults), and `README.md:104-122` (the documented usage
//! block). All four bin aliases share this one surface.

#[path = "Argument.rs"]
pub mod argument;
#[path = "Flag.rs"]
pub mod flag;
#[path = "Pattern.rs"]
pub mod pattern;
#[path = "Root.rs"]
pub mod root;
#[path = "Exclude.rs"]
pub mod exclude;
#[path = "Command.rs"]
pub mod command;
#[path = "Parallel.rs"]
pub mod parallel;
#[path = "File.rs"]
pub mod file;
