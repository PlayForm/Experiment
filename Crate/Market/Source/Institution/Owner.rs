//! Owner and constitution: policy, cost caps and repository consent.
//!
//! Grounded in `SYSTEM.md:72-78`: the owner sets priorities, hard resource
//! limits and publication policy, but is not presumed entitled to plaintext
//! encrypted to other recipients; a container does not hide secrets from its
//! host operator (`README.md:55`). These are typed authority facts, not grants.

use crate::Permission;

/// The constitution holder of one campaign.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Owner {
    /// Digest of the signed policy document (`SYSTEM.md:36`, `sql/v5_migration.sql:114`).
    pub policy_sha256: &'static str,
    /// Native-unit cost caps; never a fungible balance (`MARKET.md:5-19`).
    pub cost_caps: &'static str,
    /// Digest of the repository consent registry (`REPOSITORIES.md:13,78-83`).
    pub consent_registry_sha256: &'static str,
}

impl Owner {
    /// A reference owner with all authority fields named, none granted here.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            policy_sha256: "owner_policy_sha256",
            cost_caps: "native_resource_vector",
            consent_registry_sha256: "consent_registry_sha256",
        }
    }

    /// The owner may set priorities and hard limits (`SYSTEM.md:74`).
    #[must_use]
    pub const fn may_set_priorities() -> Permission {
        Permission::Allow
    }

    /// The owner may define publication policy and repository consent
    /// (`SYSTEM.md:74`, `PUBLISHING.md:66-70`).
    #[must_use]
    pub const fn may_set_publication_policy() -> Permission {
        Permission::Allow
    }

    /// The owner is not entitled to peer plaintext (`SYSTEM.md:75-77`).
    #[must_use]
    pub const fn may_read_peer_plaintext() -> Permission {
        Permission::Deny
    }

    /// Container partition does not hide memory or disk from the host operator
    /// (`README.md:55`, `SECURITY.md:59-64`).
    #[must_use]
    pub const fn host_operator_is_excluded() -> Permission {
        Permission::Deny
    }
}
