//! Plane 0: the one operator-facing public API.
//!
//! Grounded in `docs/03-external-api.md:3-31,32-78` and `README.md:9`: 67
//! documented HTTP operations, authenticated read-only observers, exactly two
//! signed owner-command POSTs, `snapshot_seq` reads, an opaque cursor with a
//! maximum page of 500, and metadata access where a digest alone never grants
//! bytes. Models cannot reach this plane. Only the typed counts and rules live
//! here; no route is served.

/// The Plane 0 descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Public {
    /// Number of documented public operations.
    pub operations: u8,
    /// Number of owner/operator write POSTs.
    pub owner_writes: u8,
    /// Maximum page size on a metadata projection.
    pub page_limit: u16,
}

impl Public {
    /// Build the canonical Plane 0 descriptor.
    #[must_use]
    pub const fn of() -> Self {
        Self {
            operations: crate::PUBLIC_OPERATIONS,
            owner_writes: crate::OWNER_WRITE_OPERATIONS,
            page_limit: crate::PUBLIC_PAGE_LIMIT,
        }
    }

    /// Whether a request count stays inside the documented page bound.
    #[must_use]
    pub const fn page_ok(&self, requested: u16) -> crate::Permission {
        match requested <= self.page_limit {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }

    /// The expected failure statuses of the normalized error contract.
    #[must_use]
    pub const fn statuses() -> [u16; 8] {
        [400, 401, 403, 404, 409, 412, 429, 503]
    }
}
