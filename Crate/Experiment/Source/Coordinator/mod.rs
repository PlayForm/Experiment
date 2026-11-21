//! Deterministic coordinator: owns scheduler, evidence and state.
//!
//! Grounded in `ARCHITECTURE.md:28-31,35-37,98-109`,
//! `REQUIREMENTS_V3.md` INV-001/AGT3-001/AGT3-008 and `RUNBOOK.md:10-18`.
//! No model runs in this deterministic path.

#[path = "Coordinator.rs"]
pub mod coordinator;
#[path = "Authority.rs"]
pub mod authority;
#[path = "Token.rs"]
pub mod token;
#[path = "Quota.rs"]
pub mod quota;
#[path = "Lease.rs"]
pub mod lease;
#[path = "Approval.rs"]
pub mod approval;
#[path = "Transition.rs"]
pub mod transition;
