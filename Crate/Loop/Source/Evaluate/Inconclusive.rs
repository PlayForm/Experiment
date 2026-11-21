//! A preserved inconclusive outcome.
//! Grounded in `00-system-constitution.md:54-55` (preserve failure and
//! inconclusive states) and `MARKET.md:89-91` (award null results; a low-power
//! result is not rewritten as a win).

/// An outcome where confidence could not be established.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inconclusive {
    /// Why confidence was not established.
    pub reason: String,
    /// Number of samples available, for power accounting.
    pub samples: u32,
}

impl Inconclusive {
    /// Record an inconclusive outcome.
    #[must_use]
    pub fn of(reason: &str, samples: u32) -> Self {
        Self {
            reason: reason.to_owned(),
            samples,
        }
    }

    /// An inconclusive outcome is always retained, never deleted.
    #[must_use]
    pub const fn is_preserved(&self) -> bool {
        true
    }

    /// Whether more samples might establish confidence within budget.
    #[must_use]
    pub const fn may_resample(&self) -> bool {
        self.samples > 0
    }
}
