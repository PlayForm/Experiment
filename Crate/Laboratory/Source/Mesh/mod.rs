//! Benchmark mesh facade.
//!
//! Declares the five required leaves plus the supporting `Candidate` point.
//! The full Pareto frontier, targeted implementations and separate hardware
//! classes are preserved (`docs/00:14-15`, `docs/10-benchmark-mesh.md`).

#[path = "BenchmarkMesh.rs"]
pub mod benchmark_mesh;
#[path = "Workload.rs"]
pub mod workload;
#[path = "Node.rs"]
pub mod node;
#[path = "Isolation.rs"]
pub mod isolation;
#[path = "Pareto.rs"]
pub mod pareto;
#[path = "Candidate.rs"]
pub mod candidate;

pub use benchmark_mesh::BenchmarkMesh;
pub use candidate::Candidate;
pub use isolation::Isolation;
pub use node::Node;
pub use pareto::Pareto;
pub use workload::Workload;
