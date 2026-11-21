//! The `prun` binary: one composition over the eleven re-synthesised crates.
//!
//! This is the merge point the research asks for - separate folders first,
//! then one command surface (`Research/prun_lab/README.md` Start 45-62,
//! Extend 84-117; `Research/prun_laboratory_v6_1/docs/26-acceptance-and-build-plan.md`
//! 34-58, 82-89). Every command delegates to a typed model owned by a sibling
//! crate; nothing here implements Benchmark, Runner, Campaign, Experiment,
//! Trust, Market, Laboratory, Contract, Storage, Custody or Fabric logic.
//!
//! The LLM is an untrusted proposer of a staged command. Selection, evidence,
//! scheduling and release stay with the deterministic models we call. A
//! missing config, unknown crate or absent binary is logged as a typed
//! degradation, never a panic.

#![forbid(unsafe_code)]

#[path = "Cli/mod.rs"]
pub mod cli;
#[path = "Dispatch/mod.rs"]
pub mod dispatch;
#[path = "Compose/mod.rs"]
pub mod compose;
#[path = "Report/mod.rs"]
pub mod report;
#[path = "Warning/mod.rs"]
pub mod warning;

use crate::cli::command::Command;
use crate::compose::wiring::Wiring;
use crate::report::view::View;

/// Entry point: parse, compose, render. No branch and no loop lives here.
fn main() {
    let words = std::env::args().skip(1).collect::<Vec<String>>();
    let command = Command::parse(&words);
    let option = crate::cli::option::Option::baseline();
    let config = std::fs::read_to_string(&option.controller.config).ok();
    let wiring = Wiring::compose(&command, &option, config.as_deref());
    let view = View::of(&wiring);
    view.lines().for_each(|line| println!("{line}"));
}
