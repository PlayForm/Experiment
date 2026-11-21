//! Optional OpenPGP interoperability helper.
//!
//! Grounded in `security/src/openpgp.rs:1-3` (not the native agent message
//! protocol; public keys must be imported and verified through the CA identity
//! registry before their signatures are relied upon), `security/Cargo.toml:8-16`
//! (feature-gated dependency) and `main.rs:35-43` (four CLI verbs). A missing
//! or dead helper degrades with a warning; no OpenPGP operation runs here.

#[path = "Helper.rs"]
pub mod helper;
#[path = "Command.rs"]
pub mod command;
