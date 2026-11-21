//! Oracle: trusted frozen-commit correctness, independent of patcher edits.
//!
//! Grounded in `ARCHITECTURE.md:54-55,127-129`, `REQUIREMENTS_V3.md` EXP-003/
//! EXP-006 and `playbooks/fanout-5.yaml:65-68`. An oracle change is a separate
//! reviewable mutation, never an inline edit by its evaluated candidate.

#[path = "Oracle.rs"]
pub mod oracle;
#[path = "Fixture.rs"]
pub mod fixture;
#[path = "Reference.rs"]
pub mod reference;
#[path = "Differential.rs"]
pub mod differential;
#[path = "Coverage.rs"]
pub mod coverage;
