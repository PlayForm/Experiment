//! How the loop is launched from the `prun loop` command surface.
//! Grounded in `26-acceptance-and-build-plan.md:34-58` and the `prun loop`
//! command owned by the Integration crate; this module is a typed projection
//! of that launch and never supersedes the trusted command.

#[path = "Args.rs"]
pub mod args;
#[path = "Run.rs"]
pub mod run;
#[path = "Report.rs"]
pub mod report;
