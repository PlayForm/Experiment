//! The trusted correctness oracle. A worker-supplied test never substitutes
//! for the frozen reference, differential and coverage checks.
//! Grounded in `00-system-constitution.md:29-31`, `ARCHITECTURE.md:52-55` and
//! `26-acceptance-and-build-plan.md:74-79`.

#[path = "Oracle.rs"]
pub mod oracle;
#[path = "Reference.rs"]
pub mod reference;
#[path = "Differential.rs"]
pub mod differential;
#[path = "Coverage.rs"]
pub mod coverage;
#[path = "Verdict.rs"]
pub mod verdict;
