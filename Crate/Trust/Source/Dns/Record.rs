//! Resource records in the private lab zone.
//!
//! Grounded in `security/dns/lab.internal.zone:3-9`: SOA and NS records, plus
//! A records for `ns` (172.30.30.53), `relay` (172.30.30.10),
//! `coordinator` (172.30.30.11) and `verifier` (172.30.30.12). The zone file
//! states that no sensitive key material or credential fingerprint appears in
//! DNS responses (`lab.internal.zone:9`). No query is served here.

use crate::permission::Permission;

/// A typed DNS resource record from the lab zone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Record {
    /// Start of authority (`lab.internal.zone:3`).
    Soa {
        name: &'static str,
        primary: &'static str,
        admin: &'static str,
    },
    /// Name server delegation (`lab.internal.zone:4`).
    Ns {
        name: &'static str,
        target: &'static str,
    },
    /// IPv4 address record (`lab.internal.zone:5-8`).
    A {
        name: &'static str,
        address: &'static str,
    },
}

impl Record {
    /// The complete record set in zone-file order.
    pub const ALL: [Self; 6] = [
        Self::Soa {
            name: "@",
            primary: "ns.lab.internal.",
            admin: "operator.lab.internal.",
        },
        Self::Ns {
            name: "@",
            target: "ns.lab.internal.",
        },
        Self::A {
            name: "ns",
            address: "172.30.30.53",
        },
        Self::A {
            name: "relay",
            address: "172.30.30.10",
        },
        Self::A {
            name: "coordinator",
            address: "172.30.30.11",
        },
        Self::A {
            name: "verifier",
            address: "172.30.30.12",
        },
    ];

    /// The record type as written in the zone file.
    #[must_use]
    pub const fn r#type(self) -> &'static str {
        match self {
            Self::Soa { .. } => "SOA",
            Self::Ns { .. } => "NS",
            Self::A { .. } => "A",
        }
    }

    /// Owner name of the record.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Soa { name, .. } | Self::Ns { name, .. } | Self::A { name, .. } => name,
        }
    }

    /// A record must never carry key material or credential fingerprints
    /// (`lab.internal.zone:9`).
    #[must_use]
    pub const fn may_carry_secrets() -> Permission {
        Permission::Deny
    }
}
