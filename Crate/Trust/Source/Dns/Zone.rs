//! The private lab zone definition.
//!
//! Grounded in `security/dns/lab.internal.zone:1-4`: `$ORIGIN lab.internal.`,
//! `$TTL 60`, and the SOA record `ns.lab.internal. operator.lab.internal.`
//! with serial `2026112101` and refresh/retry/expire/minimum
//! `3600 600 86400 60`. No zone file is parsed here.

use crate::DNS_TTL_SECONDS;

/// A typed private zone header and SOA definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Zone {
    /// `$ORIGIN` (`lab.internal.zone:1`).
    pub origin: &'static str,
    /// `$TTL` (`lab.internal.zone:2`).
    pub ttl: u32,
    /// SOA primary name server (`lab.internal.zone:3`).
    pub primary: &'static str,
    /// SOA responsible mailbox (`lab.internal.zone:3`).
    pub admin: &'static str,
    /// SOA serial (`lab.internal.zone:3`).
    pub serial: u64,
    /// SOA refresh, retry, expire and minimum (`lab.internal.zone:3`).
    pub timers: [u32; 4],
}

impl Zone {
    /// Construct the prototype's zone definition.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            origin: "lab.internal.",
            ttl: DNS_TTL_SECONDS,
            primary: "ns.lab.internal.",
            admin: "operator.lab.internal.",
            serial: 2_026_112_101,
            timers: [3600, 600, 86_400, 60],
        }
    }

    /// The zone is internal-only, not a public trust anchor
    /// (`named.toml:1`).
    #[must_use]
    pub const fn is_private() -> bool {
        true
    }

    /// Check a supplied TTL against the declared value.
    pub fn check_ttl(&self, found: u32) -> Result<(), crate::warning::Warning> {
        match found == self.ttl {
            true => Ok(()),
            false => Err(crate::warning::Warning::Drift {
                domain: "dns-ttl",
                expected: self.ttl,
                found,
            }),
        }
    }
}
