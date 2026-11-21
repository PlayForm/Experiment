//! The render contract: panel icons, spinner and layout, as typed data.
//!
//! Grounded in `Fn/Tui/Render.rs:11-149` and `Struct/Tui.rs:3-104`. The
//! `Frame` is supplied by the caller; no `Terminal::draw` is invoked.

/// A frame renderer's static vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Render {
    tick: usize,
}

impl Render {
    /// The ten-frame braille spinner (`Struct/Tui.rs:33`).
    pub const SPINNER: [&'static str; 10] = [
        "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏",
    ];
    /// Pending icon (`Render.rs:40`).
    pub const PENDING: &'static str = "○ ";
    /// Completed icon (`Render.rs:44`).
    pub const DONE: &'static str = "✓ ";
    /// Failed icon (`Render.rs:45`).
    pub const FAILED: &'static str = "✗ ";
    /// Index-lock timeout icon (`Render.rs:46`).
    pub const TIMEOUT: &'static str = "⚠ ";
    /// Directory panel title (`Render.rs:64`).
    pub const DIRECTORY_TITLE: &'static str = " ◈ Directories ";
    /// Log panel title when nothing is selected (`Render.rs:76`).
    pub const LOG_TITLE: &'static str = " ◈ Log ";

    /// Bind the renderer to a tick count (`Fn/Tui/mod.rs:93`).
    #[must_use]
    pub const fn build(tick: usize) -> Self {
        Self { tick }
    }

    /// Spinner glyph for the current tick (`Render.rs:30`).
    #[must_use]
    pub const fn spinner(&self) -> &'static str {
        Self::SPINNER[self.tick % Self::SPINNER.len()]
    }

    /// Running-row prefix, spinner plus `Done/Total` (`Render.rs:42`).
    #[must_use]
    pub fn running_icon(&self, done: usize, total: usize) -> String {
        format!("{} {}/{} ", self.spinner(), done, total)
    }

    /// Status-bar text when all directories are done (`Render.rs:118-127`).
    #[must_use]
    pub fn all_done_status(&self, done: usize, total: usize) -> String {
        format!(" ✓ All done ({done}/{total}) - q quit")
    }

    /// Status-bar text while work remains (`Render.rs:121-126`).
    #[must_use]
    pub fn running_status(&self, done: usize, total: usize) -> String {
        format!(
            " {} Running {}/{} - ↑↓ select  PgUp/PgDn scroll  s auto-scroll  q quit",
            self.spinner(),
            done,
            total
        )
    }
}
