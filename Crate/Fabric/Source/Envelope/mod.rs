//! Generic encrypted wire envelopes: randomized names are not authority.
//!
//! Mirrors `docs/02:29-33`, `docs/05:47-48`, `docs/08:37-42`, the four
//! authoritative envelope schemas and `README.md:22-24`. The broker sees a
//! routing token and payload size, never field names or policy expressions.
//! No encryption, signature verification or network transmission occurs here.

#[path = "Opaque.rs"]
pub mod opaque;
#[path = "Message.rs"]
pub mod message;
#[path = "Grant.rs"]
pub mod grant;
#[path = "Crosswalk.rs"]
pub mod crosswalk;
#[path = "Commitment.rs"]
pub mod commitment;
#[path = "Proof.rs"]
pub mod proof;
