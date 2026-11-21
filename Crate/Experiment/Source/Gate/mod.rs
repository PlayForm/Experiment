//! Novelty and evidence gate: deterministic verifier, advisory model only.
//!
//! Grounded in `RESEARCH_GATE.md:1-117`, `ARCHITECTURE.md:46-51`,
//! `REQUIREMENTS_V3.md` NOV-001..010 and `playbooks/fanout-5.yaml:48-60`.
//! Only versioned policy approves or rejects.

#[path = "Novelty.rs"]
pub mod novelty;
#[path = "Evidence.rs"]
pub mod evidence;
#[path = "Identity.rs"]
pub mod identity;
#[path = "Proximity.rs"]
pub mod proximity;
#[path = "Ancestry.rs"]
pub mod ancestry;
#[path = "Reason.rs"]
pub mod reason;
#[path = "Decision.rs"]
pub mod decision;
