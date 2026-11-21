//! Hickory DNS configuration descriptor.
//!
//! Grounded in `security/dns/named.toml:1-11`: Hickory DNS 0.26.x, isolated
//! authoritative zone and explicitly "not a trust anchor", listening on
//! `0.0.0.0:53`, zone `lab.internal`, `zone_type = "Primary"`, file
//! `lab.internal.zone`, and `axfr_policy = "Deny"`. No daemon is configured here.

use crate::permission::Permission;

/// A typed Hickory authoritative-server configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    /// IPv4 listen address (`named.toml:2`).
    pub listen_ipv4: &'static str,
    /// IPv6 listen addresses (`named.toml:3`: empty).
    pub listen_ipv6: &'static str,
    /// Listen port (`named.toml:4`).
    pub listen_port: u16,
    /// Zone directory (`named.toml:5`).
    pub directory: &'static str,
    /// Zone name (`named.toml:8`).
    pub zone: &'static str,
    /// Zone type (`named.toml:9`).
    pub zone_type: &'static str,
    /// Zone file (`named.toml:10`).
    pub file: &'static str,
    /// AXFR policy (`named.toml:11`).
    pub axfr_policy: &'static str,
}

impl Config {
    /// Construct the prototype's DNS configuration descriptor.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            listen_ipv4: "0.0.0.0",
            listen_ipv6: "",
            listen_port: 53,
            directory: "/etc/hickory/zones",
            zone: "lab.internal",
            zone_type: "Primary",
            file: "lab.internal.zone",
            axfr_policy: "Deny",
        }
    }

    /// The zone is not a trust anchor (`named.toml:1`).
    #[must_use]
    pub const fn is_trust_anchor() -> Permission {
        Permission::Deny
    }

    /// Zone transfers are denied (`named.toml:11`).
    #[must_use]
    pub const fn axfr_allowed() -> Permission {
        Permission::Deny
    }
}
