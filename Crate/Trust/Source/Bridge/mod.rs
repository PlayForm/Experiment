//! v4-to-v5 trust bridge: signed bids, claims, papers and scoped capabilities.
//!
//! Grounded in `Research/prun_optimization_v5/architecture/V4_TRUST_BRIDGE.md`
//! in full: role-specific message types and required evidence (`:11-31`),
//! disclosure policy (`:33-47`), revocation and key rotation (`:49-55`), and
//! the non-conflation of CA signatures with scientific validation (`:57-63`).
//! These are proposed envelope payload contracts, not wired v4 behaviour.

#[path = "Capability.rs"]
pub mod capability;
#[path = "Purpose.rs"]
pub mod purpose;
#[path = "Declassification.rs"]
pub mod declassification;
#[path = "Signature.rs"]
pub mod signature;
#[path = "Treasury.rs"]
pub mod treasury;
