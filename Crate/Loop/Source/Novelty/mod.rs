//! The deterministic four-pass novelty and evidence gate.
//! Grounded in `RESEARCH_GATE.md:1-117` (scope, exact identity, canonical
//! structural similarity and semantic similarity; a fingerprint is a hint,
//! never proof) and `ARCHITECTURE.md:46-51,75-80`.

#[path = "Identity.rs"]
pub mod identity;
#[path = "Key.rs"]
pub mod key;
#[path = "Proximity.rs"]
pub mod proximity;
#[path = "Ancestry.rs"]
pub mod ancestry;
#[path = "Decision.rs"]
pub mod decision;
