//! Why an exclusive lease was returned.

/// The documented reasons a lease stops being held.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseReason {
    /// Authoritative work completed.
    Complete,
    /// Policy revoked the holder.
    Revoked,
    /// The validity window elapsed.
    Expired,
    /// The holder was abandoned after a fault.
    Abandoned,
}

impl ReleaseReason {
    /// The declared reason name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::Abandoned => "abandoned",
        }
    }
}
