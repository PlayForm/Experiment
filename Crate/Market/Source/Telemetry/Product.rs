//! Product measurement plane and trial equivalence.
//!
//! Grounded in `TELEMETRY.md:14-16`: only controlled product execution timing
//! and counters belong here, release trials use minimal instrumentation, and
//! rich trace builds run separately and are marked non-equivalent. This is a
//! typed equivalence decision; no timing is collected.

/// The isolated product measurement plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Product {
    /// Whether the observed build was instrumented minimally as released.
    pub minimal_instrumentation: crate::Permission,
}

impl Product {
    /// Construct a product-plane descriptor.
    #[must_use]
    pub const fn of(minimal_instrumentation: crate::Permission) -> Self {
        Self {
            minimal_instrumentation,
        }
    }

    /// Rich trace builds are not equivalent to release trials
    /// (`TELEMETRY.md:15-16`).
    #[must_use]
    pub fn is_release_equivalent(&self) -> crate::Permission {
        self.minimal_instrumentation
    }

    /// The plane this value belongs to.
    #[must_use]
    pub const fn plane() -> super::Plane {
        super::Plane::Product
    }

    /// No private prompt may enter a measurement attribute
    /// (`TELEMETRY.md:7-8`).
    #[must_use]
    pub const fn may_include_prompt() -> crate::Permission {
        crate::Permission::Deny
    }
}
