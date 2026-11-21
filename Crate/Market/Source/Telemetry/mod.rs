//! Three-plane observability for the research economy.
//!
//! Grounded in `architecture/TELEMETRY.md:1-64`: control/cryptographic,
//! research/economy and product measurement planes are separated; correlation
//! uses opaque IDs; the append-only ledger remains authority even if an
//! exporter drops records; the GPUI cockpit gets read-only projections and no
//! private decryption keys. No span, metric or log is emitted or ingested.

#[path = "Plane.rs"]
pub mod plane;
#[path = "Control.rs"]
pub mod control;
#[path = "Research.rs"]
pub mod research;
#[path = "Product.rs"]
pub mod product;
#[path = "Cockpit.rs"]
pub mod cockpit;

pub use cockpit::Cockpit;
pub use control::Control;
pub use plane::Plane;
pub use product::Product;
pub use research::Research;
