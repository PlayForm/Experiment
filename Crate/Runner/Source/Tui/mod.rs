//! Ratatui frame and keyboard contracts, modelled without terminal I/O.
//!
//! Grounded in `Fn/Tui/Render.rs:11-149`, `Fn/Tui/Input.rs:5-57`, `Fn/Tui/mod.rs:25-99`
//! and `Struct/Tui.rs:3-104`. No crossterm call, no alternate screen, no draw.

#[path = "Frame.rs"]
pub mod frame;
#[path = "Key.rs"]
pub mod key;
#[path = "Input.rs"]
pub mod input;
#[path = "Render.rs"]
pub mod render;
