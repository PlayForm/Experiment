//! Two-phase artifact custody facade.
//!
//! Authorization and reservation precede access; delivery, verification and
//! commit follow it. Invalid, stale, uncommitted or orphan deliveries are
//! quarantined, never reported as success
//! (`docs/19:22-51`; `policy/v61-state-machines.yaml:3-12,31-58`).

#[path = "Phase.rs"]
pub mod phase;
#[path = "Request.rs"]
pub mod request;
#[path = "Receipt.rs"]
pub mod receipt;
#[path = "Holder.rs"]
pub mod holder;
#[path = "Qualification.rs"]
pub mod qualification;
#[path = "Possession.rs"]
pub mod possession;
#[path = "Transfer.rs"]
pub mod transfer;
