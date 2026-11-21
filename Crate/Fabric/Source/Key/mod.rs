//! Dedicated key custody, not one universal master secret.
//!
//! The eight logical authorities come from `docs/04:1-24`; the rejection of a
//! single reconstruction seed from `docs/01:41-48` and `README.md:30-33`; the
//! HKDF domain separation rule from `docs/04:25-29`; capability verification
//! from `docs/04:31-44`; and threat limits from `docs/04:52-60`. No key bytes
//! exist in this crate: every value is a handle, epoch or commitment.

#[path = "Hierarchy.rs"]
pub mod hierarchy;
#[path = "Root.rs"]
pub mod root;
#[path = "Service.rs"]
pub mod service;
#[path = "Message.rs"]
pub mod message;
#[path = "Data.rs"]
pub mod data;
#[path = "Rotation.rs"]
pub mod rotation;
#[path = "Recovery.rs"]
pub mod recovery;
#[path = "NonExportable.rs"]
pub mod nonexportable;
