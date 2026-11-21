//! Three separate truth domains: control signal, product signal, measurement.
//!
//! Grounded in `TELEMETRY.md:1-25` (three forms of truth, separate service
//! namespaces and endpoints), `REQUIREMENTS_V3.md` TEL-001/TEL-007 and
//! `sql/v3_migration.sql:152-164` (`domain IN ('control','product',
//! 'measurement')`). Losing an OTel span never mutates measurement evidence.

/// The telemetry domain a signal belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Plane {
    /// Coordinator, gate, builder, oracle and evaluator operational signals.
    Control,
    /// Optional diagnostic instrumentation of the product binary.
    Product,
    /// Authoritative raw timing facts referenced by the ledger.
    Measurement,
}

impl Plane {
    /// All three domains, in schema order.
    pub const ALL: [Self; 3] = [Self::Control, Self::Product, Self::Measurement];

    /// The exact token used in `telemetry_references.domain`.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Control => "control",
            Self::Product => "product",
            Self::Measurement => "measurement",
        }
    }

    /// The service namespace, distinct for each plane (`TELEMETRY.md:20-25`).
    #[must_use]
    pub const fn service_namespace(self) -> &'static str {
        match self {
            Self::Control => "lab.control",
            Self::Product => "prun.product",
            Self::Measurement => "prun.measurement",
        }
    }

    /// No telemetry plane is the authoritative event ledger
    /// (`REQUIREMENTS_V3.md` TEL-007).
    #[must_use]
    pub const fn is_event_ledger(self) -> bool {
        false
    }

    /// Whether this plane may sample or drop data without affecting raw
    /// timing truth. Control and product spans may; measurement may not.
    #[must_use]
    pub const fn may_drop(self) -> bool {
        matches!(self, Self::Control | Self::Product)
    }
}
