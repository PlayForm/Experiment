//! Sealed-holdout exposure policy.
//!
//! Grounded in `schemas/research-case.v5.schema.json:101-103`,
//! `MARKET.md:9-13,92-94` and `AGENTS.md:5,12`: exposure is a scientific access
//! quota, not money; the replication designer sees released public results
//! only, and repeated peeks require a workload be designated development
//! forever.

use crate::Warning;

/// How a case may use its held-out data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Holdout {
    /// Sealed until confirmation; never shown during search (`schema:102`).
    Sealed,
    /// Development-only workloads, designated forever after repeated peeks
    /// (`schema:102`, `AGENTS.md:5`).
    DevelopmentOnly,
    /// No holdout for this case (`schema:102`).
    None,
}

impl Holdout {
    /// All holdout policies.
    pub const ALL: [Self; 3] = [Self::Sealed, Self::DevelopmentOnly, Self::None];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "sealed" => Ok(Self::Sealed),
            "development_only" => Ok(Self::DevelopmentOnly),
            "none" => Ok(Self::None),
            other => Err(Warning::InvalidValue {
                field: "holdout policy",
                detail: format!("unknown policy {other}"),
            }),
        }
    }

    /// The wire label of this policy.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sealed => "sealed",
            Self::DevelopmentOnly => "development_only",
            Self::None => "none",
        }
    }

    /// Exposure is never purchasable (`MARKET.md:11-13`).
    #[must_use]
    pub const fn may_be_purchased() -> crate::Permission {
        crate::Permission::Deny
    }

    /// A sealed holdout cannot be peeked at during search
    /// (`AGENTS.md:12`, `MARKET.md:92-94`).
    #[must_use]
    pub const fn may_be_peeked(self) -> crate::Permission {
        match self {
            Self::DevelopmentOnly => crate::Permission::Allow,
            Self::Sealed | Self::None => crate::Permission::Deny,
        }
    }
}
