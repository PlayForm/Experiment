//! The independent statistical evaluator. It preserves regressions and
//! inconclusive outcomes and never promotes on a single noisy win.
//! Grounded in `ARCHITECTURE.md:58-59,87-89`, `RESEARCH_GATE.md:76-95` and
//! `experiment::evaluate::verdict`.

#[path = "Evaluator.rs"]
pub mod evaluator;
#[path = "Protocol.rs"]
pub mod protocol;
#[path = "Pareto.rs"]
pub mod pareto;
#[path = "Regression.rs"]
pub mod regression;
#[path = "Inconclusive.rs"]
pub mod inconclusive;
#[path = "Invalid.rs"]
pub mod invalid;
