//! Reservation lifecycle states.
//!
//! Grounded in `sql/v5_migration.sql:158-165`: `reserved`,
//! `partially_consumed`, `settled`, `released`, `expired`. The immutable
//! transaction ledger wins over this projection (`:151`), and an expired lease
//! may be reclaimed only after verifying the worker can no longer commit
//! (`OPERATIONS.md:46`).

use crate::Warning;

/// State of a resource reservation projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LeaseState {
    /// Capacity reserved atomically (`sql:160`).
    Reserved,
    /// Some reserved capacity consumed (`sql:161`).
    PartiallyConsumed,
    /// Settlement consumed actual spend and returned unused capacity (`sql:162`).
    Settled,
    /// Reservation released (`sql:163`).
    Released,
    /// Reservation expired at an epoch boundary (`sql:164`, `MARKET.md:118`).
    Expired,
}

impl LeaseState {
    /// All reservation states in table order.
    pub const ALL: [Self; 5] = [
        Self::Reserved,
        Self::PartiallyConsumed,
        Self::Settled,
        Self::Released,
        Self::Expired,
    ];

    /// Decode the SQL label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "reserved" => Ok(Self::Reserved),
            "partially_consumed" => Ok(Self::PartiallyConsumed),
            "settled" => Ok(Self::Settled),
            "released" => Ok(Self::Released),
            "expired" => Ok(Self::Expired),
            other => Err(Warning::InvalidValue {
                field: "lease state",
                detail: format!("unknown state {other}"),
            }),
        }
    }

    /// The SQL label of this state.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Reserved => "reserved",
            Self::PartiallyConsumed => "partially_consumed",
            Self::Settled => "settled",
            Self::Released => "released",
            Self::Expired => "expired",
        }
    }

    /// Whether capacity is still held by this reservation.
    #[must_use]
    pub const fn is_open(self) -> bool {
        matches!(self, Self::Reserved | Self::PartiallyConsumed)
    }
}
