//! The two-timescale feedback model.
//! Grounded in `17-v61-constitution.md:45-51` (immediate private feedback is
//! that worker's authorized projection; delayed cross-repo dissemination
//! needs declassification, license and embargo checks and a fresh experiment)
//! and `PUBLISHING.md:105-113` (abstract knowledge card, no measured effect).

/// Fast private capsule separated from slow sanctioned knowledge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TwoTimescale {
    /// Immediate private capsule; absent until evaluation completes.
    pub fast: Option<super::capsule::Capsule>,
    /// Slow, sanctioned abstract card; absent until declassification.
    pub slow: Option<String>,
}

impl TwoTimescale {
    /// Both timescales start empty.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fast: None,
            slow: None,
        }
    }

    /// Attach a fast private capsule.
    #[must_use]
    pub fn with_fast(mut self, capsule: super::capsule::Capsule) -> Self {
        self.fast = Some(capsule);
        self
    }

    /// Whether slow knowledge still requires a declassification grant.
    #[must_use]
    pub const fn needs_declassification(&self) -> bool {
        self.slow.is_none()
    }

    /// Whether the fast capsule is available to the worker now.
    #[must_use]
    pub const fn fast_ready(&self) -> bool {
        self.fast.is_some()
    }
}

impl Default for TwoTimescale {
    fn default() -> Self {
        Self::new()
    }
}
