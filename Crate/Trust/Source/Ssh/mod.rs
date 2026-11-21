//! OpenSSH short-lived certificate support.
//!
//! Grounded in `security/src/ssh_ca.rs:1-72`: a pure-Rust Ed25519 CA key held
//! offline issues short-lived user certificates; the principal is a dedicated
//! non-root account, validity is `now-10 .. now+600` seconds, and no
//! forwarding, agent forwarding or PTY extensions are granted. This is not an
//! SSH server (`ssh_ca.rs:1-3`). No key is generated and no certificate is signed.

#[path = "Ca.rs"]
pub mod ca;
#[path = "Certificate.rs"]
pub mod certificate;
#[path = "Option.rs"]
pub mod option;
