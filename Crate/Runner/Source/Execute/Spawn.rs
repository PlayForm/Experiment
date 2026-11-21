//! The `sh -c` process-spawn contract.
//!
//! Grounded in `Fn/Binary/Command/Parallel/Process.rs:20-32` and
//! `Fn/Binary/Command/Sequential.rs:57-62`: program `sh`, argv `["-c",
//! trimmed_command]`, `current_dir` = target directory, both streams piped.
//! An empty/trimmed-empty command is rejected with an `io::Error` analogue.

use crate::fault::Fault;

/// A spawn description: data only, never executed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Spawn {
    command: String,
    directory: String,
}

impl Spawn {
    /// The shell program (`Process.rs:27`, `Sequential.rs:57`).
    pub const PROGRAM: &'static str = "sh";
    /// The shell flag (`Process.rs:28`, `Sequential.rs:58`).
    pub const FLAG: &'static str = "-c";

    /// Validate the command as `Process::Fn` does (`Process.rs:21-24`),
    /// returning a typed fault instead of an `io::Error`.
    pub fn build(command: String, directory: String) -> Result<Self, Fault> {
        match command.trim().is_empty() {
            true => Err(Fault::Process {
                command,
                detail: "Empty command provided".to_owned(),
            }),
            false => Ok(Self { command, directory }),
        }
    }

    /// The command after the SUT's `trim()` (`Process.rs:21`).
    #[must_use]
    pub fn trimmed(&self) -> &str {
        self.command.trim()
    }

    /// argv passed after the program (`Process.rs:28`).
    #[must_use]
    pub fn argv(&self) -> [String; 2] {
        [Self::FLAG.to_owned(), self.trimmed().to_owned()]
    }

    /// Working directory (`Process.rs:29`, `Sequential.rs:59`).
    #[must_use]
    pub fn directory(&self) -> &str {
        &self.directory
    }
}
