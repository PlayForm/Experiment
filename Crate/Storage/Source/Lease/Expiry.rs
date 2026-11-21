//! The validity window of a lease, compared on pinned UTC strings.

/// An acquired and an expiry UTC timestamp.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expiry {
    /// Acquisition time.
    pub acquired_utc: String,
    /// Time after which the lease is expired.
    pub expires_utc: String,
}

impl Expiry {
    /// Declare a validity window.
    #[must_use]
    pub fn of(acquired_utc: &str, expires_utc: &str) -> Self {
        Self {
            acquired_utc: acquired_utc.to_owned(),
            expires_utc: expires_utc.to_owned(),
        }
    }

    /// Classify the window at a supplied UTC instant. ISO-8601 timestamps
    /// compare lexicographically, so no clock is read here.
    #[must_use]
    pub fn observe(&self, now: &str) -> super::state::State {
        match now < self.expires_utc {
            true => super::state::State::Active,
            false => super::state::State::Expired,
        }
    }

    /// Report a malformed window without panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        let missing = [
            ("acquired utc", self.acquired_utc.is_empty()),
            ("expires utc", self.expires_utc.is_empty()),
        ];
        let empty = missing.into_iter().filter_map(|(field, absent)| {
            absent.then(|| crate::warning::Warning::Malformed {
                domain: field,
                detail: "a lease window must be bounded".to_owned(),
            })
        });
        let reversed = (self.expires_utc < self.acquired_utc).then(|| {
            crate::warning::Warning::Malformed {
                domain: "lease window",
                detail: "expiry precedes acquisition".to_owned(),
            }
        });
        empty.chain(reversed).collect()
    }
}
