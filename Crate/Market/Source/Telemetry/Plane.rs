//! The three observability planes.
//!
//! Grounded in `TELEMETRY.md:3-16`: control/cryptographic, research/economy
//! and product measurement. No plaintext payload or private prompt may appear
//! in an attribute, and no private opinion may appear in research telemetry.

/// One observability plane of the research economy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Plane {
    /// Task leases, CA issuance, identities, envelopes, transitions, commits
    /// (`TELEMETRY.md:5-8`).
    Control,
    /// Bids, reservations, spend, calibration and paper events (`:9-13`).
    Research,
    /// Controlled product execution timing and counters (`:14-16`).
    Product,
}

impl Plane {
    /// All three planes in document order.
    pub const ALL: [Self; 3] = [Self::Control, Self::Research, Self::Product];

    /// Stable plane name.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Control => "control_cryptographic",
            Self::Research => "research_economy",
            Self::Product => "product_measurement",
        }
    }

    /// No plane may carry plaintext payloads or private prompts
    /// (`TELEMETRY.md:7-8,13`).
    #[must_use]
    pub const fn may_carry_plaintext() -> crate::Permission {
        crate::Permission::Deny
    }

    /// The append-only ledger, not an exporter, remains authority
    /// (`TELEMETRY.md:24-25`).
    #[must_use]
    pub const fn is_authoritative() -> crate::Permission {
        crate::Permission::Deny
    }
}
