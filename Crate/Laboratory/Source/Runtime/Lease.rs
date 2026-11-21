//! Fenced leases over exclusive physical resources.
//!
//! Grounded in `docs/06-state-machines.md:92-102`,
//! `docs/04-internal-api.md:14-16` and `docs/25-failure-analysis.md` F11/F29:
//! `lease_id + fencing_token + resource_id + owner_job_id` are binding; a
//! timeout increments the fence before another worker may produce authoritative
//! output; a held benchmark lease must be exclusive over the physical topology.
//! A stale report can never commit evidence.

/// A fenced, exclusive lease on a named resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// Unique lease identifier.
    pub id: String,
    /// Monotonic fencing token.
    pub fence: u64,
    /// Physical resource the lease covers.
    pub resource: String,
    /// Owning job or attempt identifier.
    pub owner: String,
}

impl Lease {
    /// Grant a lease, refusing an empty resource with a warning.
    #[must_use]
    pub fn of(
        id: String,
        fence: u64,
        resource: String,
        owner: String,
    ) -> Result<Self, crate::Warning> {
        match resource.is_empty() {
            true => Err(crate::Warning::InvalidValue {
                field: "lease resource",
                detail: "a lease must name its physical resource".to_owned(),
            }),
            false => Ok(Self {
                id,
                fence,
                resource,
                owner,
            }),
        }
    }

    /// Whether a report carrying `token` may commit authoritative output.
    #[must_use]
    pub fn accepts(&self, token: u64) -> crate::Permission {
        match token == self.fence {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }

    /// Issue the next lease after a timeout, incrementing the fence.
    #[must_use]
    pub fn renew(&self, next_owner: String) -> Self {
        Self {
            id: self.id.clone(),
            fence: self.fence + 1,
            resource: self.resource.clone(),
            owner: next_owner,
        }
    }
}
