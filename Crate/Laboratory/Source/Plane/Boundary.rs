//! Boundary X: the offline worker, explicitly not a fourth trusted plane.
//!
//! Grounded in `README.md:17-20`, `docs/01-topology-and-boundaries.md:28-56`
//! and `docs/17-v61-constitution.md:11-18`: the model process runs in an
//! offline namespace with no network, no service addresses, no credentials and
//! no trusted API routes. It sees only a scoped source snapshot, a task,
//! allowed local tools, private feedback and an output scratch directory. An
//! offline model may infer it is being benchmarked; OS-level capability, not
//! its ignorance, enforces the boundary. This enum makes the "not a plane"
//! rule structural.

/// A trust boundary in the laboratory topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Boundary {
    /// Human observation and signed owner commands.
    PlaneZero,
    /// Trusted control and orchestration.
    PlaneOne,
    /// Product/SUT execution evidence.
    PlaneTwo,
    /// The offline untrusted model container.
    OfflineX,
}

impl Boundary {
    /// Whether this boundary is itself a trusted plane.
    #[must_use]
    pub const fn trusted_plane(self) -> crate::Permission {
        match self {
            Self::PlaneZero | Self::PlaneOne | Self::PlaneTwo => crate::Permission::Allow,
            Self::OfflineX => crate::Permission::Deny,
        }
    }

    /// Whether a component at this boundary may hold a credential or route.
    #[must_use]
    pub const fn holds_authority(self) -> crate::Permission {
        match self {
            Self::PlaneOne | Self::PlaneTwo => crate::Permission::Allow,
            Self::PlaneZero | Self::OfflineX => crate::Permission::Deny,
        }
    }

    /// The number of trusted planes; Boundary X is not counted.
    #[must_use]
    pub const fn count() -> u8 {
        crate::TRUSTED_PLANES
    }
}
