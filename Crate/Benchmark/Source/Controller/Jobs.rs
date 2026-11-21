//! Parallel build-job count (`--jobs`).
//!
//! Grounded in `prun_lab.py:653-654,684-685,415-421`: one job builds variants
//! serially; more jobs use a bounded thread pool. Only builds are parallel;
//! measurements never are.

use crate::Fault;

/// Number of concurrent builder jobs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Jobs(u32);

impl Jobs {
    /// Controller default (prun_lab.py:654).
    pub const DEFAULT: Self = Self(1);

    /// Validate the documented floor of one.
    pub fn new(value: u32) -> Result<Self, Fault> {
        (value >= 1)
            .then_some(Self(value))
            .ok_or_else(|| Fault::Malformed {
                domain: "jobs",
                field: "jobs".to_owned(),
                detail: "must be >= 1".to_owned(),
            })
    }

    /// Worker count handed to the pool.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Whether builds run serially (prun_lab.py:415-421).
    #[must_use]
    pub const fn is_serial(self) -> bool {
        self.0 == 1
    }
}
