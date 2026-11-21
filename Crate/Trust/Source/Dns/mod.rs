//! Private DNS zone from `security/dns/`.
//!
//! Grounded in `security/dns/lab.internal.zone:1-9` and
//! `security/dns/named.toml:1-11`: an isolated authoritative zone that is not a
//! trust anchor, with AXFR denied and no key material in responses. No DNS
//! server is started and no query is answered here.

#[path = "Zone.rs"]
pub mod zone;
#[path = "Record.rs"]
pub mod record;
#[path = "Config.rs"]
pub mod config;
