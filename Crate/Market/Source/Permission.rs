//! Explicit two-variant permission: the typed form of a research-economy boolean.
//!
//! House style says a `bool` field is a two-variant enum waiting to be written.
//! `Deny` is the default so an omitted authority, consent or declassification
//! field fails closed (`README.md:44-49`, `V4_TRUST_BRIDGE.md:28-31`,
//! `PUBLISHING.md:66-70`). No capability is granted by this type; it only
//! records a decision a trusted caller already made.

use std::fmt;

use serde::{Deserialize, Deserializer};

/// A capability or permission decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Permission {
    /// The operation is allowed by a separately signed, versioned policy.
    Allow,
    /// The operation is denied; callers must degrade, not panic.
    Deny,
}

impl Permission {
    /// Deserialize a JSON boolean into `Allow` / `Deny`.
    pub fn from_bool<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bool::deserialize(deserializer).map(Self::from)
    }

    /// View the decision as a boolean at an explicit wire boundary.
    #[must_use]
    pub const fn as_bool(self) -> bool {
        matches!(self, Self::Allow)
    }

    /// Combine two permissions, keeping the stricter decision.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Allow, Self::Allow) => Self::Allow,
            _ => Self::Deny,
        }
    }

    /// The reason a decision was recorded, for warnings and audit lineage.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Allow => "explicitly permitted by signed versioned policy",
            Self::Deny => "denied by default; requires explicit signed approval",
        }
    }
}

impl Default for Permission {
    fn default() -> Self {
        Self::Deny
    }
}

impl From<bool> for Permission {
    fn from(flag: bool) -> Self {
        match flag {
            true => Self::Allow,
            false => Self::Deny,
        }
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        })
    }
}
