//! The sequential execution strategy, modelled as typed transitions.
//!
//! Grounded in `Fn/Binary/Command/Sequential.rs:14-135`: target directories
//! keep walk order (`:38`), commands keep user order (`:47`), an index-lock
//! timeout skips the whole directory via `continue 'directories`
//! (`:52-55`), and `AllDone` is always emitted last (`:134`).

use crate::walk::entry::Entry;

/// The sequential strategy's static contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sequential {
    pattern: String,
}

impl Sequential {
    /// Event kinds this strategy emits, in order of first appearance.
    pub const EMITS: [&'static str; 5] = [
        "JobStarted",
        "Line",
        "JobProgress",
        "JobFinished",
        "AllDone",
    ];
    /// The loop label used to skip a directory on lock timeout (`:38,54`).
    pub const SKIP_LABEL: &'static str = "directories";

    /// Bind the strategy to the pattern used for target matching.
    #[must_use]
    pub fn build(pattern: String) -> Self {
        Self { pattern }
    }

    /// The ordered target list: parent of every entry whose final component
    /// equals the pattern (`Sequential.rs:26-36`).
    #[must_use]
    pub fn targets(&self, candidates: &[Entry]) -> Vec<String> {
        candidates
            .iter()
            .filter_map(|entry| match entry.name() == Some(self.pattern.as_str()) {
                true => entry.parent(),
                false => None,
            })
            .collect()
    }

    /// The progress counter after command `index` (`Sequential.rs:119-126`).
    #[must_use]
    pub const fn progress(index: usize, total: usize) -> (usize, usize) {
        (index + 1, total)
    }

    /// Terminal event emitted once every directory has been visited.
    #[must_use]
    pub const fn terminal() -> &'static str {
        "AllDone"
    }
}
