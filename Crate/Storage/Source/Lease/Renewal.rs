//! A fence-incrementing lease renewal.

use super::owner::Owner;

/// The next fence and holder after a timeout or handover.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Renewal {
    /// New monotonic fencing token.
    pub fence: u64,
    /// New holder, which cannot reuse the old token.
    pub owner: Owner,
}

impl Renewal {
    /// Issue a renewal from the previous fence.
    #[must_use]
    pub fn of(previous_fence: u64, owner: Owner) -> Self {
        Self {
            fence: previous_fence + 1,
            owner,
        }
    }

    /// Describe the renewal for a diagnostic.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("fence={}, owner={}", self.fence, self.owner.id)
    }
}
