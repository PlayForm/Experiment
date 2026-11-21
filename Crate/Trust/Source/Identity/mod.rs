//! Ed25519 identity issuance and separated principals.
//!
//! Grounded in `security/src/crypto.rs:32-48` (`PublicIdentity`,
//! `AttestedIdentity`), `crypto.rs:109-129` (validation: name, revocation,
//! CA signature, validity window), `storage.rs:163-220` (issuance) and
//! `V4_TRUST_BRIDGE.md:11-31` (separated role principals). No key is generated
//! and no signature is verified here.

#[path = "Identity.rs"]
pub mod identity;
#[path = "Certificate.rs"]
pub mod certificate;
#[path = "Revoke.rs"]
pub mod revoke;
#[path = "Principal.rs"]
pub mod principal;
