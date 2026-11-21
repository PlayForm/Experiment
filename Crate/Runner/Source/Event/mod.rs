//! The observed event stream shared by the CLI printer and the TUI.
//!
//! Grounded in `Struct/Event.rs:2-8` (the six-variant enum) and consumed by
//! `Struct/Binary/Command.rs:35-74`, `Fn/Tui/mod.rs:39-81` and
//! `Fn/Tui/Render.rs:29-136`. This is a type catalogue only; no channel is
//! created and no event is emitted.

#[path = "Event.rs"]
pub mod event;
#[path = "Kind.rs"]
pub mod kind;
#[path = "Payload.rs"]
pub mod payload;
