//! Director: the parent reasoning agent as a replaceable reasoning edge.
//!
//! Grounded in `ARCHITECTURE.md:38-40,94-96`, `AGENT_CONTRACTS.md:7`,
//! `REQUIREMENTS_V3.md` AGT3-010 and `playbooks/fanout-5.yaml:96-104`.
//! Wide READ view and proposal capability only; never evidence edits.

#[path = "Director.rs"]
pub mod director;
#[path = "View.rs"]
pub mod view;
#[path = "Proposal.rs"]
pub mod proposal;
#[path = "Arbitration.rs"]
pub mod arbitration;
