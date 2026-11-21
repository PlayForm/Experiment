//! The three `ArgAction::SetTrue` switches declared by the SUT.
//!
//! Grounded in `Fn/Binary/Command.rs:14-36`: `File` (short F, long "File",
//! display order 1), `Parallel` (short P, order 2) and `Tui` (short T, order
//! 3). Each is read with `ArgMatches::get_flag` in `Option.rs:38-40`.

/// A presence-only switch; clap sets it true, never stores a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Flag {
    /// `-F/--File`: target files, not directories containing a pattern.
    File,
    /// `-P/--Parallel`: execute across all found directories at once.
    Parallel,
    /// `-T/--Tui`: launch the interactive panel dashboard.
    Tui,
}

impl Flag {
    /// Declaration order, matching `display_order` 1..=3.
    pub const ALL: [Self; 3] = [Self::File, Self::Parallel, Self::Tui];

    /// Short spelling (`Command.rs:15,23,31`).
    #[must_use]
    pub const fn short(self) -> char {
        match self {
            Self::File => 'F',
            Self::Parallel => 'P',
            Self::Tui => 'T',
        }
    }

    /// Long spelling and clap `Arg::new` id (`Command.rs:16,24,32`).
    #[must_use]
    pub const fn long(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Parallel => "Parallel",
            Self::Tui => "Tui",
        }
    }

    /// clap `display_order` (`Command.rs:18,26,34`).
    #[must_use]
    pub const fn display_order(self) -> u8 {
        match self {
            Self::File => 1,
            Self::Parallel => 2,
            Self::Tui => 3,
        }
    }

    /// The help text used by `--help` (`Command.rs:19,27,35`).
    #[must_use]
    pub const fn help(self) -> &'static str {
        match self {
            Self::File => "Target files directly instead of directories containing a pattern.",
            Self::Parallel => "Execute commands in parallel across all found directories.",
            Self::Tui => "Launch the interactive TUI panel dashboard.",
        }
    }
}
