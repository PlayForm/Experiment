//! Length-prefixed JSON-over-TLS mTLS relay.
//!
//! Grounded in `security/src/relay.rs:1-4` (not HTTP; never holds HPKE private
//! keys), `:26-39` (frame cap and typed request/response), `:74-91` (u32 length
//! prefix), `:99-160` (publish/fetch/ack handling), `:162-234` (rustls server
//! and client, peer from certificate fingerprint, serialized filesystem) and
//! `security/README.md:5-9`. No socket is opened and no TLS is negotiated here.

#[path = "Relay.rs"]
pub mod relay;
#[path = "Frame.rs"]
pub mod frame;
#[path = "Wire.rs"]
pub mod wire;
#[path = "Tls.rs"]
pub mod tls;
#[path = "Mtls.rs"]
pub mod mtls;
