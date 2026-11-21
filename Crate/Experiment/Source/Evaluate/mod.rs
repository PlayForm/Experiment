//! Evaluate: versioned statistical protocol, non-LLM.
//!
//! Grounded in `ARCHITECTURE.md:58-59,88-89`, `REQUIREMENTS_V3.md` EXP-005/
//! EXP-009, `RESEARCH_GATE.md:76-95` and `playbooks/fanout-5.yaml:80-83`.
//! Inconclusive and failure are preserved, never rewritten as wins.

#[path = "Evaluator.rs"]
pub mod evaluator;
#[path = "Verdict.rs"]
pub mod verdict;
#[path = "Regression.rs"]
pub mod regression;
#[path = "Win.rs"]
pub mod win;
#[path = "Inconclusive.rs"]
pub mod inconclusive;
#[path = "Invalid.rs"]
pub mod invalid;
#[path = "Protocol.rs"]
pub mod protocol;
