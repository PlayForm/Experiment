//! The `-P/--Parallel` switch resolved into an execution strategy.
//!
//! Grounded in `Fn/Binary/Command.rs:21-28` (SetTrue flag),
//! `Struct/Binary/Command/Option.rs:39` (`get_flag("Parallel")`), and
//! `Struct/Binary/Command.rs:33,37-72`, where the value selects
//! `Fn::Binary::Command::Parallel::Fn` or `...::Sequential::Fn`.

/// Which execution strategy the entry closure dispatches to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Parallel {
    /// Walk order preserved, one command at a time (`Sequential.rs:38`).
    Sequential,
    /// Rayon classification + Tokio workers (`Parallel.rs:27-146`).
    Concurrent,
}

impl Parallel {
    /// Resolve clap's `get_flag("Parallel")` boolean into a strategy.
    #[must_use]
    pub const fn from_flag(enabled: bool) -> Self {
        match enabled {
            true => Self::Concurrent,
            false => Self::Sequential,
        }
    }

    /// Whether the concurrent strategy was selected.
    #[must_use]
    pub const fn is_concurrent(self) -> bool {
        matches!(self, Self::Concurrent)
    }
}
