//! Interface: typed mirror of the five v3 interface JSON schemas.
//!
//! Grounded in `interfaces/event-envelope.v3.schema.json`,
//! `proposal.v3.schema.json`, `attestation.v3.schema.json`,
//! `feedback-capsule.v3.schema.json`, `agent-turn.v3.schema.json`, every
//! matching `examples/*.json`, and `ARCHITECTURE.md:98-109`. Schema version 3.

#[path = "Envelope.rs"]
pub mod envelope;
#[path = "Proposal.rs"]
pub mod proposal;
#[path = "Attestation.rs"]
pub mod attestation;
#[path = "Capsule.rs"]
pub mod capsule;
#[path = "Turn.rs"]
pub mod turn;
#[path = "EventKind.rs"]
pub mod kind;
