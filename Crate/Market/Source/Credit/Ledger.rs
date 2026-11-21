//! Immutable resource-transaction kinds.
//!
//! Grounded in `MARKET.md:70-79` (authorization, reservation, usage,
//! settlement, refund, expiration and manual adjustment, each with a causal
//! transaction ID, signed actor and rationale) and `sql/v5_migration.sql:127-135`
//! (`fund`, `reserve`, `consume`, `release`, `expire`, `adjust`). The ledger is
//! append-only and its immutability triggers are `:267-278`. No transaction is
//! written here.

use crate::Warning;

/// One kind of resource transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Ledger {
    /// Owner funding of the campaign (`sql:130`, `MARKET.md:72`).
    Fund,
    /// Atomic reservation decreasing available capacity (`sql:131`, `MARKET.md:76`).
    Reserve,
    /// Metered actual consumption up to the reservation (`sql:132`, `MARKET.md:77`).
    Consume,
    /// Refund or release of unused capacity (`sql:133`, `MARKET.md:77`).
    Release,
    /// Epoch-boundary expiration returning capacity (`sql:134`, `MARKET.md:118`).
    Expire,
    /// Signed manual adjustment with rationale (`sql:135`, `MARKET.md:72`).
    Adjust,
}

impl Ledger {
    /// All transaction kinds in table order.
    pub const ALL: [Self; 6] = [
        Self::Fund,
        Self::Reserve,
        Self::Consume,
        Self::Release,
        Self::Expire,
        Self::Adjust,
    ];

    /// Decode the SQL label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "fund" => Ok(Self::Fund),
            "reserve" => Ok(Self::Reserve),
            "consume" => Ok(Self::Consume),
            "release" => Ok(Self::Release),
            "expire" => Ok(Self::Expire),
            "adjust" => Ok(Self::Adjust),
            other => Err(Warning::InvalidValue {
                field: "transaction kind",
                detail: format!("unknown kind {other}"),
            }),
        }
    }

    /// The SQL label of this kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fund => "fund",
            Self::Reserve => "reserve",
            Self::Consume => "consume",
            Self::Release => "release",
            Self::Expire => "expire",
            Self::Adjust => "adjust",
        }
    }

    /// Every ledger row is immutable (`sql:267-278`).
    #[must_use]
    pub const fn is_immutable() -> crate::Permission {
        crate::Permission::Allow
    }

    /// Overages are rejected or escalated, never silently absorbed
    /// (`MARKET.md:77`, `OPERATIONS.md:36` `budget_exceeded`).
    #[must_use]
    pub const fn may_overdraw(self) -> crate::Permission {
        crate::Permission::Deny
    }
}
