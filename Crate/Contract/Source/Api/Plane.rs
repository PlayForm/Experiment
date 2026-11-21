//! The two authenticated API planes.
//!
//! Plane 0 is the operator-facing external API; Plane 1 is the trusted
//! internal service API (`docs/01-topology-and-boundaries.md`,
//! `docs/03-external-api.md:1-11`, `docs/04-internal-api.md:1-7`). Boundary X
//! is an offline worker and is deliberately not a third API plane: a model has
//! no route, no credential and no control API in its namespace
//! (`policy/v61-roles.json:4-12`).

use crate::Permission;

/// An authenticated HTTP plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Plane {
    /// External, operator/observer-only surface (`HTTP_INDEX.md:9-75`).
    External,
    /// Internal, trusted-service-only surface (`HTTP_INDEX.md:76-128`).
    Internal,
}

impl Plane {
    /// Wire spelling used by the HTTP index.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::External => "public",
            Self::Internal => "internal",
        }
    }

    /// Ordering rank: 0 is external, 1 is internal.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::External => 0,
            Self::Internal => 1,
        }
    }

    /// Transport required by this plane.
    #[must_use]
    pub const fn transport(self) -> &'static str {
        match self {
            Self::External => "loopback-or-mTLS",
            Self::Internal => "mTLS-scoped-service",
        }
    }

    /// Whether an LLM worker may ever reach this plane.
    #[must_use]
    pub const fn worker_access(self) -> Permission {
        match self {
            Self::External => Permission::Deny,
            Self::Internal => Permission::Deny,
        }
    }

    /// Decode the HTTP index domain column.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "public" => Some(Self::External),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }
}
