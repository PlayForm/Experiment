//! The observed outcome of one spawned process.
//!
//! Grounded in `Fn/Binary/Command/Parallel/Process.rs:34-64`: stdout is
//! collected line-by-line (each line re-appended with `\n`), stderr is
//! captured, and a non-success status becomes an error carrying both the
//! status and trimmed stderr. `Sequential.rs:78-126` emits stderr lines
//! separately; that difference is a `Contract` observation, not behaviour
//! performed here.

use crate::fault::Fault;

/// A process result that the SUT would turn into `Event::Line` values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    command: String,
    stdout: String,
    stderr: String,
    status: i32,
}

impl Process {
    /// Record an outcome. No process is run to obtain these values.
    #[must_use]
    pub const fn build(command: String, stdout: String, stderr: String, status: i32) -> Self {
        Self {
            command,
            stdout,
            stderr,
            status,
        }
    }

    /// `ExitStatus::success()` (`Process.rs:56`, `Sequential.rs:99`).
    #[must_use]
    pub const fn success(&self) -> bool {
        self.status == 0
    }

    /// Captured stdout, with trailing newlines (`Process.rs:40-41`).
    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Captured stderr, trimmed only when formatting an error.
    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Exit code as an owned value.
    #[must_use]
    pub const fn status(&self) -> i32 {
        self.status
    }

    /// Non-empty stdout lines, in order (`Parallel.rs:99-108`).
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.stdout.lines().filter(|line| !line.trim().is_empty())
    }

    /// The success/failure projection (`Process.rs:56-64`).
    pub fn output(&self) -> Result<&str, Fault> {
        match self.success() {
            true => Ok(&self.stdout),
            false => Err(Fault::Process {
                command: self.command.clone(),
                detail: format!(
                    "Command failed with status {}. Stderr: {}",
                    self.status,
                    self.stderr.trim()
                ),
            }),
        }
    }
}
