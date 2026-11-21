//! Signed HPKE envelope model.
//!
//! Grounded in `security/src/crypto.rs`: recipient-only HPKE with independent
//! Ed25519 signatures (`:1-2`), domain separation (`:25-27`), fixed 32-byte keys
//! (`:81-84`), the protected header and envelope (`:49-67`), signature coverage
//! (`:68-73,130-137`), size and time checks (`:146,188-214`) and replay
//! mark-once (`:230-236`). No algorithm runs here.

#[path = "Key.rs"]
pub mod key;
#[path = "Envelope.rs"]
pub mod envelope;
#[path = "Hpke.rs"]
pub mod hpke;
#[path = "Signature.rs"]
pub mod signature;
#[path = "Nonce.rs"]
pub mod nonce;
