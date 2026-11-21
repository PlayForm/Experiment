//! Dashboard facade.
//!
//! Declares only the four required leaves. All views are plane-separated
//! projections of the public API and none can claim an unpublished agent's
//! internal thought process (`docs/12-dashboard.md`, `docs/24:60-71`).

#[path = "View.rs"]
pub mod view;
#[path = "Panel.rs"]
pub mod panel;
#[path = "Metric.rs"]
pub mod metric;
#[path = "Redaction.rs"]
pub mod redaction;

pub use metric::Metric;
pub use panel::Panel;
pub use redaction::Redaction;
pub use view::View;
