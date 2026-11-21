//! The release transition of an exclusive lease.

use super::release_reason::ReleaseReason;

/// A typed release with the fence that authorized it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    /// Lease being released.
    pub lease_id: String,
    /// Fence presented by the holder.
    pub fence: u64,
    /// Why the lease was released.
    pub reason: ReleaseReason,
}

impl Release {
    /// Declare a release.
    #[must_use]
    pub fn of(lease_id: &str, fence: u64, reason: ReleaseReason) -> Self {
        Self {
            lease_id: lease_id.to_owned(),
            fence,
            reason,
        }
    }

    /// Describe the release.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{} fence={} {}", self.lease_id, self.fence, self.reason.label())
    }
}
