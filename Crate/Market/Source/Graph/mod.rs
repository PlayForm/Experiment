//! The typed research-economy graph from `graphs/research-economy.mmd`.
//!
//! The 27-line Mermaid graph is read as data: the owner funds treasury and
//! identity, the exchange fans out to private researchers, signed claims drive
//! the patch/oracle/builder/benchmark/evaluator chain, publication yields a
//! signed corpus, the integrator loops back, and two telemetry planes feed a
//! GPUI observatory. No graph is rendered or executed here.

#[path = "Node.rs"]
pub mod node;
#[path = "Edge.rs"]
pub mod edge;
#[path = "Flow.rs"]
pub mod flow;

pub use edge::Edge;
pub use flow::Flow;
pub use node::Node;
