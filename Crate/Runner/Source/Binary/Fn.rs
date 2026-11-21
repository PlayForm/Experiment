//! The main async closure's pipeline and dispatch table.
//!
//! Grounded in `Struct/Binary/Command.rs:24-76`: build options, read the
//! statically cached clap args, build execution options, open an mpsc channel
//! of capacity 64, choose TUI vs CLI and parallel vs sequential, then drain
//! the event stream.

use crate::cli::parallel::Parallel;

/// The boxed-future workflow held by the entry struct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fn;

impl Fn {
    /// Channel capacity (`Command.rs:35`).
    pub const CHANNEL_CAPACITY: usize = 64;

    /// Named stages of the workflow, in order (`Command.rs:29-73`).
    #[must_use]
    pub const fn stages() -> &'static [&'static str] {
        &[
            "Command::Fn()",
            "Option::Struct::Fn",
            "Entry::Struct::Fn",
            "mpsc::channel(64)",
            "dispatch",
            "drain events",
        ]
    }

    /// The executor selected by `-P` (`Command.rs:37-42,68-72`).
    #[must_use]
    pub const fn dispatch(mode: Parallel) -> &'static str {
        match mode {
            Parallel::Concurrent => "Fn::Binary::Command::Parallel::Fn",
            Parallel::Sequential => "Fn::Binary::Command::Sequential::Fn",
        }
    }

    /// Which surface consumes the receiver (`Command.rs:37-74`).
    #[must_use]
    pub const fn surface(tui: bool) -> &'static str {
        match tui {
            true => "Fn::Tui::Fn",
            false => "CLI printer task",
        }
    }
}
