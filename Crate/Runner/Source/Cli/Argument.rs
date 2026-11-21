//! The resolved command-line value object: the typed mirror of clap's
//! `ArgMatches` as read by `Struct/Binary/Command/Option.rs:22-60`.
//!
//! Grounding: that file supplies the eight fields (`Command`, `Exclude`,
//! `File`, `Parallel`, `Pattern`, `Root`, `Separator`, `Tui`); `Command.rs:8-75`
//! supplies the seven arg definitions; `README.md:112-122` the usage line.
//! Set-true flags are held as `Flag` values rather than stored bools, so a
//! bool never becomes a field of this record.

use crate::cli::{
    command::Command, exclude::Exclude, file::File, flag::Flag, parallel::Parallel, pattern::Pattern,
    root::Root,
};

/// The fully resolved CLI contract handed to the execution stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    command: Command,
    exclude: Exclude,
    pattern: Pattern,
    root: Root,
    flags: Vec<Flag>,
    separator: char,
}

impl Argument {
    /// Assemble a validated argument set from its typed parts.
    #[must_use]
    pub fn build(
        command: Command,
        exclude: Exclude,
        pattern: Pattern,
        root: Root,
        flags: Vec<Flag>,
        separator: char,
    ) -> Self {
        Self {
            command,
            exclude,
            pattern,
            root,
            flags,
            separator,
        }
    }

    /// Clap defaults: root ".", the single default exclusion glob, no flags.
    #[must_use]
    pub fn from_defaults(
        command: Command,
        pattern: Pattern,
        separator: char,
    ) -> Self {
        Self::build(
            command,
            Exclude::default_patterns(),
            pattern,
            Root::default(),
            Vec::new(),
            separator,
        )
    }

    /// `-F` resolved (`Option.rs:38` -> `Entry.rs:50`).
    #[must_use]
    pub fn file(&self) -> File {
        File::from_flag(self.flags.contains(&Flag::File))
    }

    /// `-P` resolved (`Option.rs:39` -> `Command.rs:33`).
    #[must_use]
    pub fn parallel(&self) -> Parallel {
        Parallel::from_flag(self.flags.contains(&Flag::Parallel))
    }

    /// `-T` resolved (`Option.rs:40` -> `Command.rs:32`).
    #[must_use]
    pub fn tui(&self) -> bool {
        self.flags.contains(&Flag::Tui)
    }

    /// OS path separator captured at construction (`Command.rs:23`).
    #[must_use]
    pub const fn separator(&self) -> char {
        self.separator
    }

    /// The command list (`Option.rs:52-56`).
    #[must_use]
    pub const fn command(&self) -> &Command {
        &self.command
    }

    /// The exclusion patterns (`Option.rs:42-47`).
    #[must_use]
    pub const fn exclude(&self) -> &Exclude {
        &self.exclude
    }

    /// The required pattern (`Option.rs:48-51`).
    #[must_use]
    pub const fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// The root path (`Option.rs:41`).
    #[must_use]
    pub const fn root(&self) -> &Root {
        &self.root
    }
}
