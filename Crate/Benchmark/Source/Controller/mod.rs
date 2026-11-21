//! Typed subcommand surface and dispatch, modelling README.md:45-78 and
//! `prun_lab.py::main` (646-697). Dispatch returns data describing the next
//! stage; it never spawns a process or writes a file.

#[path = "Command.rs"]
pub mod command;
#[path = "Init.rs"]
pub mod init;
#[path = "Plan.rs"]
pub mod plan;
#[path = "Build.rs"]
pub mod build;
#[path = "Bench.rs"]
pub mod bench;
#[path = "All.rs"]
pub mod all;
#[path = "Option.rs"]
pub mod option;
#[path = "Jobs.rs"]
pub mod jobs;
#[path = "ParallelRunners.rs"]
pub mod parallel_runners;

#[path = "Dispatch.rs"]
pub mod dispatch;

pub use all::All;
pub use bench::Bench;
pub use build::Build;
pub use command::Command;
pub use dispatch::Dispatch;
pub use init::Init;
pub use jobs::Jobs;
pub use option::Option;
pub use parallel_runners::ParallelRunners;
pub use plan::Plan;
