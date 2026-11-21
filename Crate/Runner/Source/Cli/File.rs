//! The `-F/--File` switch resolved into a target-selection mode.
//!
//! Grounded in `Fn/Binary/Command.rs:14-19` (SetTrue flag) and
//! `Fn/Binary/Command/Entry.rs:50`, where `Option.File` selects
//! `is_file()` candidates instead of all non-excluded entries.

/// What a walk returns to the execution stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum File {
    /// Default: keep entries whose name matches the pattern, run in parents.
    Directories,
    /// `-F`: keep entries that are files themselves (`Entry.rs:50`).
    Files,
}

impl File {
    /// Resolve clap's `get_flag("File")` boolean into a mode.
    #[must_use]
    pub const fn from_flag(enabled: bool) -> Self {
        match enabled {
            true => Self::Files,
            false => Self::Directories,
        }
    }

    /// Whether the walk filters to files (`Entry.rs:50`).
    #[must_use]
    pub const fn targets_files(self) -> bool {
        matches!(self, Self::Files)
    }
}
