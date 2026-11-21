//! The terminal area and its three-panel split, as data.
//!
//! Grounded in `Fn/Tui/Render.rs:12-27`: `Frame::area()`, a vertical split of
//! `Min(3)` plus `Length(1)`, then a horizontal split of `30%`/`70%`; and in
//! `:89`, where the log height loses two border rows. Ratatui is never
//! constructed and nothing is drawn.

/// A frame's measured area.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Frame {
    width: u16,
    height: u16,
}

impl Frame {
    /// Minimum height of the directory panel (`Render.rs:16`).
    pub const MIN_LIST_HEIGHT: u16 = 3;
    /// Height of the status bar (`Render.rs:16`).
    pub const STATUS_HEIGHT: u16 = 1;
    /// Directory-panel width percentage (`Render.rs:21`).
    pub const LIST_PERCENT: u16 = 30;
    /// Log-panel width percentage (`Render.rs:21`).
    pub const LOG_PERCENT: u16 = 70;
    /// Border rows subtracted when computing log scroll (`Render.rs:89`).
    pub const LOG_PADDING: u16 = 2;
    /// Click hit-box width (`Input.rs:54`).
    pub const CLICK_COLUMN_MAX: u16 = 40;
    /// Click hit-box first row (`Input.rs:54`).
    pub const CLICK_ROW_MIN: u16 = 2;

    /// Record a frame size.
    #[must_use]
    pub const fn build(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Frame width.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Frame height.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    /// Vertical split heights: `[directory+log, status]` (`Render.rs:16-17`).
    #[must_use]
    pub const fn vertical(&self) -> (u16, u16) {
        (self.height.saturating_sub(Self::STATUS_HEIGHT), Self::STATUS_HEIGHT)
    }

    /// Horizontal split widths: `[directories, log]` (`Render.rs:21-22`).
    #[must_use]
    pub const fn horizontal(&self) -> (u16, u16) {
        let list = (self.width as u32 * Self::LIST_PERCENT as u32 / 100) as u16;

        (list, self.width.saturating_sub(list))
    }

    /// Visible log lines, matching `Area.height - 2` (`Render.rs:89`).
    #[must_use]
    pub const fn log_capacity(&self) -> usize {
        self.height.saturating_sub(Self::LOG_PADDING) as usize
    }
}
