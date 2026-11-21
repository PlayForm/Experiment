//! A policy or supervisor epoch, never reused after a restart.

/// A monotonic epoch number stamped on authority tokens.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Epoch {
    /// Monotonic epoch counter.
    pub number: u64,
}

impl Epoch {
    /// The initial epoch.
    #[must_use]
    pub fn origin() -> Self {
        Self { number: 1 }
    }

    /// Declare an epoch.
    #[must_use]
    pub fn of(number: u64) -> Self {
        Self { number }
    }

    /// Advance to the next epoch.
    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            number: self.number + 1,
        }
    }

    /// Compare a presented epoch with the current one.
    #[must_use]
    pub fn classify(&self, presented: &Self) -> EpochState {
        match presented.number.cmp(&self.number) {
            std::cmp::Ordering::Equal => EpochState::Current,
            std::cmp::Ordering::Less => EpochState::Stale,
            std::cmp::Ordering::Greater => EpochState::Superseded,
        }
    }

    /// Whether a token bearing this epoch is still authoritative.
    #[must_use]
    pub fn accepts(&self, presented: &Self) -> crate::permission::Permission {
        match self.classify(presented) {
            EpochState::Current => crate::permission::Permission::Allow,
            EpochState::Stale | EpochState::Superseded => crate::permission::Permission::Deny,
        }
    }
}
