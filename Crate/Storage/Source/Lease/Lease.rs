//! A fenced exclusive lease over a physical or storage resource.

use super::expiry::Expiry;
use super::owner::Owner;
use super::release::Release;
use super::release_reason::ReleaseReason;
use super::renewal::Renewal;
use super::state::State;

/// One lease as recorded by the v6 and v6.1 schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// Lease identifier.
    pub id: String,
    /// Resource under exclusive control.
    pub resource: String,
    /// Current holder.
    pub owner: Owner,
    /// Current fencing token.
    pub fence: u64,
    /// Validity window.
    pub expiry: Expiry,
    /// Current lease state.
    pub state: State,
}

impl Lease {
    /// Declare a lease, requiring a positive fence and a named resource.
    #[must_use]
    pub fn of(
        id: &str,
        resource: &str,
        owner: Owner,
        fence: u64,
        expiry: Expiry,
        state: State,
    ) -> Self {
        Self {
            id: id.to_owned(),
            resource: resource.to_owned(),
            owner,
            fence,
            expiry,
            state,
        }
    }

    /// Whether a report carrying this token may commit authoritative output.
    #[must_use]
    pub fn accepts(&self, token: u64) -> crate::permission::Permission {
        let fenced = match token == self.fence {
            true => crate::permission::Permission::Allow,
            false => crate::permission::Permission::Deny,
        };
        fenced.and(self.state.authoritative())
    }

    /// Warn when a stale token is presented.
    #[must_use]
    pub fn fence_finding(&self, token: u64) -> Option<crate::warning::Warning> {
        match self.accepts(token) {
            crate::permission::Permission::Allow => None,
            crate::permission::Permission::Deny => Some(crate::warning::Warning::Fenced {
                lease: self.id.clone(),
                expected: self.fence,
                found: token,
            }),
        }
    }

    /// Issue the next lease after a timeout.
    #[must_use]
    pub fn renew(&self, next_owner: Owner) -> Renewal {
        Renewal::of(self.fence, next_owner)
    }

    /// Return the lease with a reason.
    #[must_use]
    pub fn release(&self, reason: ReleaseReason) -> Release {
        Release::of(&self.id, self.fence, reason)
    }

    /// Classify the lease at a supplied UTC instant.
    #[must_use]
    pub fn observe(&self, now: &str) -> State {
        self.expiry.observe(now)
    }

    /// Report malformed lease declarations without panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        let empty = [
            ("lease id", self.id.is_empty()),
            ("lease resource", self.resource.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, absent)| {
            absent.then(|| crate::warning::Warning::Malformed {
                domain: field,
                detail: "a lease must name itself and its resource".to_owned(),
            })
        });
        let bad_fence = (self.fence == 0).then(|| crate::warning::Warning::Fenced {
            lease: self.id.clone(),
            expected: 1,
            found: self.fence,
        });
        empty
            .chain(bad_fence)
            .chain(self.owner.validate())
            .chain(self.expiry.validate())
            .collect()
    }
}
