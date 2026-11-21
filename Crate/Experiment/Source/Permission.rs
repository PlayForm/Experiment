//! Explicit two-variant permission, the typed form of a JSON boolean.
//!
//! House style says a `bool` field is a two-variant enum waiting to be written.
//! This is that enum, shared by authority, director, patcher and gate models.
//! `Deny` is the default so an omitted security field fails closed
//! (`REQUIREMENTS_V3.md` INV-002, `ARCHITECTURE.md:119-125`).
//!
//! Exception declared: the `from_bool` deserializer lives beside the enum
//! because it is the JSON encoding of this one type and has no independent
//! lifecycle.

use std::fmt;

use serde::{Deserialize, Deserializer};

/// A permission or capability decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Permission {
    /// The operation is allowed by the declared policy.
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

    /// View the decision as a boolean at an explicit boundary.
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

    /// The reason a denial is recorded, for lineage and warnings.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Allow => "explicitly permitted by versioned policy",
            Self::Deny => "denied by default; requires explicit approval",
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
