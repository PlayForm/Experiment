//! The institution graph and principal separation of the v5 research economy.
//!
//! Grounded in `architecture/SYSTEM.md:32-65` (institution graph and raw
//! notebook isolation), `:72-102` (authority versus knowledge), `:104-127`
//! (case lifecycle and decision states), `graphs/research-economy.mmd:1-27`,
//! and `architecture/OPERATIONS.md:35-49` (typed failure modes). Each leaf is a
//! typed value or a refusal; no institution here runs, signs, funds or
//! publishes anything.

#[path = "Graph.rs"]
pub mod graph;
#[path = "Owner.rs"]
pub mod owner;
#[path = "Treasury.rs"]
pub mod treasury;
#[path = "Exchange.rs"]
pub mod exchange;
#[path = "Oracle.rs"]
pub mod oracle;
#[path = "Benchmark.rs"]
pub mod benchmark;
#[path = "Publisher.rs"]
pub mod publisher;
#[path = "Integrator.rs"]
pub mod integrator;
#[path = "Adapter.rs"]
pub mod adapter;
#[path = "Capability.rs"]
pub mod capability;

pub use adapter::Adapter;
pub use benchmark::Benchmark;
pub use capability::Capability;
pub use exchange::Exchange;
pub use graph::Graph;
pub use integrator::Integrator;
pub use oracle::Oracle;
pub use owner::Owner;
pub use publisher::Publisher;
pub use treasury::Treasury;
