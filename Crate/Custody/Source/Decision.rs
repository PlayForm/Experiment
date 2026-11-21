//! The outcome of a license or rights review.
//!
//! Missing, ambiguous, incompatible or jurisdiction-sensitive terms become
//! `needs-human-review` or `prohibited` and fail closed for that use
//! (`docs/20:8-24`). This value records a decision; it never gives one.

/// A conservative rights determination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Terms were mechanically resolved and permit the use.
    Compatible,
    /// Terms were mechanically resolved and forbid the use.
    Incompatible,
    /// An authorized human owner must decide; wire spelling `needs-human-review`.
    Pending,
    /// Use is prohibited pending a separately approved review record.
    Prohibited,
}

impl Decision {
    /// Whether the determination permits propagation without review.
    #[must_use]
    pub fn permit(self) -> crate::permission::Permission {
        match self {
            Self::Compatible => crate::permission::Permission::Allow,
            Self::Incompatible | Self::Pending | Self::Prohibited => {
                crate::permission::Permission::Deny
            }
        }
    }

    /// The wire spelling used by `v61-license-grant` and compatibility examples.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Incompatible => "incompatible",
            Self::Pending => "needs-human-review",
            Self::Prohibited => "prohibited",
        }
    }
}
