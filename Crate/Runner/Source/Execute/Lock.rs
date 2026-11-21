//! The `.git/index.lock` wait contract.
//!
//! Grounded in `Fn/Binary/Command/Index/Lock.rs:5-15` (constants) and
//! `:31-70`: absent lock succeeds immediately, a lock older than the stale
//! age is removed, otherwise poll with exponential backoff up to 30 s.
//! Constants and transitions are data; no file or timer is touched.

/// Tunables observed by the real lock waiter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Lock {
    timeout_ms: u64,
    poll_initial_ms: u64,
    poll_max_ms: u64,
    stale_age_secs: u64,
}

impl Lock {
    /// Lock file relative to the repository root (`Lock.rs:34`).
    pub const LOCK_FILE: &'static str = ".git/index.lock";
    /// The SUT's constants (`Lock.rs:6,9,12,15`).
    pub const DEFAULT: Self = Self {
        timeout_ms: 30_000,
        poll_initial_ms: 50,
        poll_max_ms: 500,
        stale_age_secs: 30,
    };

    /// Override the timings for a characterization variant.
    #[must_use]
    pub const fn build(
        timeout_ms: u64,
        poll_initial_ms: u64,
        poll_max_ms: u64,
        stale_age_secs: u64,
    ) -> Self {
        Self {
            timeout_ms,
            poll_initial_ms,
            poll_max_ms,
            stale_age_secs,
        }
    }

    /// Total wait budget (`Lock.rs:6`).
    #[must_use]
    pub const fn timeout_ms(&self) -> u64 {
        self.timeout_ms
    }

    /// First poll delay (`Lock.rs:9`).
    #[must_use]
    pub const fn poll_initial_ms(&self) -> u64 {
        self.poll_initial_ms
    }

    /// Backoff ceiling (`Lock.rs:12`).
    #[must_use]
    pub const fn poll_max_ms(&self) -> u64 {
        self.poll_max_ms
    }

    /// Stale threshold (`Lock.rs:15`).
    #[must_use]
    pub const fn stale_age_secs(&self) -> u64 {
        self.stale_age_secs
    }

    /// Double the delay, capped (`Lock.rs:64`). Saturating multiplication
    /// keeps the model total even for absurd overrides.
    #[must_use]
    pub fn next_interval(&self, current_ms: u64) -> u64 {
        current_ms.saturating_mul(2).min(self.poll_max_ms)
    }

    /// Timeout test (`Lock.rs:57`).
    #[must_use]
    pub const fn expired(&self, elapsed_ms: u64) -> bool {
        elapsed_ms >= self.timeout_ms
    }

    /// Stale-lock test (`Lock.rs:44`).
    #[must_use]
    pub const fn stale(&self, age_secs: u64) -> bool {
        age_secs >= self.stale_age_secs
    }
}
